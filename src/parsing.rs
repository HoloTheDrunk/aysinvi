use crate::error::*;

use std::{path::Path, str::FromStr};

use {
    pest::{
        error::{Error, ErrorVariant},
        iterators::{Pair, Pairs},
        Parser, Span,
    },
    strum_macros::EnumString,
};

#[derive(Debug)]
pub enum SourceCode {
    File(String),
    Content(String),
}

#[derive(Parser)]
#[grammar = "../pest/grammar.pest"]
pub struct AyParser;

/// Node containing a `Span` of code and the corresponding AST
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct AyNode<'span, Inner: Node> {
    span: Span<'span>,
    inner: Inner,
}

pub trait Node {}

/// A statement is anything that cannot be expected to return a value.
#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Statement<'s> {
    FunDec {
        name: String,
        args: Vec<String>,
        body: Vec<AyNode<'s, Statement<'s>>>,
    },
    VarDec {
        names: Vec<String>,
        values: Vec<AyNode<'s, Expr<'s>>>,
    },
    Expr(AyNode<'s, Expr<'s>>),
    If {
        cond: AyNode<'s, Expr<'s>>,
        then: Vec<AyNode<'s, Statement<'s>>>,
        otherwise: Vec<AyNode<'s, Statement<'s>>>,
    },
    Loop {
        cond: Option<AyNode<'s, Expr<'s>>>,
        body: Vec<AyNode<'s, Statement<'s>>>,
    },
}
impl Node for Statement<'_> {}

/// An expression is anything that is or returns a value.
#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Expr<'s> {
    FunCall {
        name: String,
        args: Vec<AyNode<'s, Expr<'s>>>,
    },
    Array {
        items: Vec<AyNode<'s, Expr<'s>>>,
    },
    Comparison {
        left: Box<AyNode<'s, Expr<'s>>>,
        right: Box<AyNode<'s, Expr<'s>>>,
        operator: ComparisonOperator,
    },
    Number(i64),
    String(String),
    Ident(String),
    Negated(Box<AyNode<'s, Expr<'s>>>),
}
impl Node for Expr<'_> {}

#[derive(Debug, EnumString)]
#[repr(i64)]
pub enum Multiplier {
    #[strum(serialize = "melo")]
    Double = 2,
    #[strum(serialize = "pxelo")]
    Triple = 3,
}

#[derive(Debug, EnumString, PartialEq, Eq, Clone)]
pub enum ComparisonOperator {
    #[strum(serialize = "teng")]
    Equals,
}

/// Pushes new error onto stacktrace or returns pred(pair).
fn handle<'s, F, T>(parent: &Pair<'s, Rule>, pair: Pair<'s, Rule>, pred: &F) -> Result<T, Trace>
where
    F: Fn(Pair<'s, Rule>) -> Result<T, Trace>,
    T: 's
{
    let (span, rule) = (parent.as_span(), parent.as_rule());
    pred(pair).map_err(|mut trace| {
        trace.push::<PestError>(
            Stage::Parsing,
            Error::new_from_span(
                ErrorVariant::ParsingError {
                    positives: vec![rule],
                    negatives: vec![],
                },
                span,
            )
            .into(),
        );
        trace
    })
}

// FIXME: work out lifetime issues (e.g. by adding a lifetime to T)
fn handle_iter<'s, F, T>(
    parent: &Pair<'s, Rule>,
    iter: &mut Pairs<'s, Rule>,
    pred: &F,
) -> Result<Vec<T>, Trace>
where
    F: Fn(Pair<'s, Rule>) -> Result<T, Trace>,
    T: 's
{
    iter.map(|item| handle(parent, item, pred))
        .collect::<Result<Vec<T>, Trace>>()
}

macro_rules! fields {
    ($pair:ident |> $children:ident $(: $($field:ident),*)?) => {
        let mut $children = $pair.clone().into_inner();

        $(
            $(
                let $field = $children
                    .next()
                    .ok_or_else(|| Trace::new::<PestError>(
                        Stage::Parsing,
                        Error::new_from_span(
                            ErrorVariant::ParsingError {
                                positives: vec![$pair.as_rule()],
                                negatives: vec![]
                            },
                            $pair.as_span()
                        ).into()
                    ))?;
            )*
        )?
    };
}

fn build_ast_from_expr<'p, 'n>(pair: Pair<'p, Rule>) -> Result<AyNode<'n, Expr<'n>>, Trace> {
    match pair.as_rule() {
        Rule::expr => build_ast_from_expr(pair.into_inner().next().unwrap()),
        Rule::negation => Ok(AyNode {
            span: pair.as_span().clone(),
            inner: Expr::Negated(Box::new(handle(
                &pair.clone(),
                pair.into_inner().next().unwrap(),
                &build_ast_from_expr,
            )?)),
        }),
        Rule::fun_call => {
            fields!(pair |> children: name);

            let name = name.as_span().as_str().to_owned();
            let args = handle_iter(&pair, &mut children, &build_ast_from_expr)?;

            Ok(AyNode {
                span: pair.as_span().clone(),
                inner: Expr::FunCall { name, args },
            })
        }
        Rule::array => {
            fields!(pair |> children: items);

            let items = handle_iter(&pair, &mut items.into_inner(), &build_ast_from_expr)?;

            Ok(AyNode {
                span: pair.as_span().clone(),
                inner: Expr::Array { items },
            })
        }
        Rule::comparison => {
            fields!(pair |> children: left, right, comparison);

            let left = handle(&pair, left, &build_ast_from_expr)?;
            let right = handle(&pair, right, &build_ast_from_expr)?;
            let operator = ComparisonOperator::from_str(comparison.as_str()).map_err(|_| {
                Trace::new_from_message(
                    Stage::Parsing,
                    &pair,
                    format!("Unimplemented comparison operator: `{comparison}`"),
                )
            })?;

            Ok(AyNode {
                span: pair.as_span().clone(),
                inner: Expr::Comparison {
                    left: Box::new(left),
                    right: Box::new(right),
                    operator,
                },
            })
        }
        Rule::number => {
            let span = pair.as_span();
            let mut elems = span.as_str().split_whitespace();
            let number = elems.next().unwrap();

            // Bit unnecessary but better be safe than sorry
            let mult = if let Some(mult) = elems.next() {
                Multiplier::from_str(mult).map_err(|_| {
                    Trace::new::<PestError>(
                        Stage::Parsing,
                        Error::new_from_span(
                            ErrorVariant::CustomError {
                                message: format!("Unimplemented multiplier: `{mult}`"),
                            },
                            span,
                        )
                        .into(),
                    )
                })? as i64
            } else {
                1
            };

            let result = i64::from_str_radix(number, 8).map_err(|_| {
                Trace::new::<PestError>(
                    Stage::Parsing,
                    Error::new_from_span(
                        ErrorVariant::ParsingError {
                            positives: vec![Rule::number],
                            negatives: vec![],
                        },
                        span,
                    )
                    .into(),
                )
            })? * mult;

            Ok(AyNode {
                span,
                inner: Expr::Number(result),
            })
        }
        Rule::string => Ok(AyNode {
            span: pair.as_span().clone(),
            inner: Expr::String(pair.as_span().as_str().to_owned()),
        }),
        Rule::ident | Rule::fun_ident => Ok(AyNode {
            span: pair.as_span().clone(),
            inner: Expr::Ident(pair.as_span().as_str().to_owned()),
        }),
        rule => Err(Trace::new::<PestError>(
            Stage::AstBuilding,
            Error::new_from_span(
                ErrorVariant::CustomError {
                    message: format!("Missing expression-generating rule `{:?}` handling", rule),
                },
                pair.as_span(),
            )
            .into(),
        )),
    }
}

fn build_ast_from_statement<'p>(pair: Pair<'p, Rule>) -> Result<AyNode<'p, Statement<'p>>, Trace> {
    match pair.as_rule() {
        Rule::expr => Ok(AyNode {
            span: pair.as_span(),
            inner: Statement::Expr(handle(&pair.clone(), pair, &build_ast_from_expr)?),
        }),
        Rule::fun_dec => {
            let span = pair.as_span();

            fields!(pair |> children: name);

            let name = name.as_span().as_str().to_owned();

            let args = children.next().map_or(vec![], |args| {
                args.into_inner()
                    .map(|arg| arg.as_span().as_str().to_owned())
                    .collect::<Vec<String>>()
            });

            let body = children.next().map_or(Ok(vec![]), |body| {
                handle_iter(&pair, &mut body.into_inner(), &build_ast_from_statement)
            })?;

            Ok(AyNode {
                span,
                inner: Statement::FunDec { name, args, body },
            })
        }
        Rule::var_dec => {
            let span = pair.as_span();

            let mut idents = Vec::<Pair<Rule>>::new();
            let mut values = Vec::<Pair<Rule>>::new();

            pair.into_inner().for_each(|child| {
                if child.as_rule() == Rule::ident {
                    idents.push(child);
                } else {
                    values.push(child);
                }
            });

            if idents.len() != values.len() {
                return Err(Trace::new::<PestError>(
                    Stage::Parsing,
                    Error::new_from_span(
                        ErrorVariant::ParsingError {
                            positives: vec![Rule::var_dec],
                            negatives: vec![],
                        },
                        span,
                    )
                    .into(),
                ));
            }

            Ok(AyNode {
                span,
                inner: Statement::VarDec {
                    names: idents
                        .iter()
                        .map(|ident| ident.as_span().as_str().to_owned())
                        .collect(),

                    values: values
                        .iter()
                        .map(|value| build_ast_from_expr(value.clone()))
                        .collect::<Result<Vec<AyNode<Expr>>, Trace>>()?,
                },
            })
        }
        Rule::if_block => {
            fields!(pair |> children: cond, then);

            let cond = build_ast_from_expr(cond)?;

            let then = handle_iter(&pair, &mut then.into_inner(), &build_ast_from_statement)?;

            // TODO: Refactor this
            // The else case is not mandatory
            if let Some(otherwise) = children.next() {
                let otherwise = handle_iter(
                    &pair,
                    &mut otherwise.into_inner(),
                    &build_ast_from_statement,
                )?;

                Ok(AyNode {
                    span: pair.as_span(),
                    inner: Statement::If {
                        cond,
                        then,
                        otherwise,
                    },
                })
            } else {
                Ok(AyNode {
                    span: pair.as_span(),
                    inner: Statement::If {
                        cond,
                        then,
                        otherwise: vec![],
                    },
                })
            }
        }
        Rule::loop_block => {
            fields!(pair |> children);

            let mut child = children.next().unwrap();

            let (cond, body) = if children.peek().is_none() {
                (
                    None,
                    handle_iter(&pair, &mut child.into_inner(), &build_ast_from_statement)?,
                )
            } else {
                (
                    Some(handle(&pair, child, &build_ast_from_expr)?),
                    handle_iter(
                        &pair,
                        &mut children.next().unwrap().into_inner(),
                        &build_ast_from_statement,
                    )?,
                )
            };

            Ok(AyNode {
                span: pair.as_span(),
                inner: Statement::Loop { cond, body },
            })
        }
        Rule::statement => Ok(build_ast_from_statement(pair.into_inner().next().unwrap())?),
        rule => Err(Trace::new::<PestError>(
            Stage::AstBuilding,
            Error::new_from_span(
                ErrorVariant::CustomError {
                    message: format!("Missing statement-generating rule `{:?}` handling", rule),
                },
                pair.as_span(),
            )
            .into(),
        )),
    }
}

pub fn parse<'s>(source: &SourceCode) -> Result<Vec<AyNode<'s, Statement<'s>>>, Trace> {
    let mut ast: Vec<AyNode<Statement>> = vec![];

    let (mut path, content) = match source {
        SourceCode::File(path) => {
            let unparsed_file = std::fs::read_to_string(path.as_str())
                .unwrap_or_else(|_| panic!("Cannot read file at `{path}`"));
            (Some(path), unparsed_file)
        }
        SourceCode::Content(content) => (None, content.clone()),
    };

    let pairs = AyParser::parse(Rule::program, content.as_str()).map_err(PestError::from)?;

    for pair in pairs.clone() {
        recursive_print(Some(&pair), 0);
    }

    for pair in pairs {
        match pair.as_rule() {
            Rule::mod_use => {
                if let Some(ref mut path) = path {
                    // Bit of a nightmare but it seems to work
                    let parent = Path::new(path).parent().unwrap().to_str().unwrap();

                    let path = format!(
                        "{parent}/{}",
                        pair.clone()
                            .into_inner()
                            .map(|child| match child.as_rule() {
                                Rule::possessive => format!(
                                    "{}/",
                                    child.as_str().strip_suffix("yä").unwrap_or_else(|| child
                                        .as_str()
                                        .strip_suffix('ä')
                                        .unwrap())
                                ),
                                Rule::ident => format!("{}.ay", child.as_str()),
                                _ => unreachable!(),
                            })
                            .collect::<String>()
                    );

                    eprintln!("Using {path}");
                    ast.extend(parse(&SourceCode::File(path.clone()))?);
                } else {
                    return Err(Trace::new::<PestError>(
                        Stage::AstBuilding,
                        Error::new_from_span(
                            ErrorVariant::CustomError {
                                message: "Missing script directory information".to_owned(),
                            },
                            pair.as_span(),
                        )
                        .into(),
                    ));
                }
            }
            Rule::statement => ast.push(build_ast_from_statement(pair)?),
            Rule::EOI => {}
            unknown_rule => Err(Error::new_from_span(
                ErrorVariant::CustomError {
                    message: format!("Unknown rule: {:?}", unknown_rule),
                },
                pair.as_span(),
            ))
            .map_err(PestError::from)?,
        }
    }

    Ok(ast)
}

pub fn recursive_print(cur: Option<&Pair<Rule>>, depth: u8) {
    if let Some(node) = cur {
        let rule = node.as_rule();

        // TODO: simplify this using .repeat()
        let indent = (0..depth)
            .map(|_| "\x1b[32m|   \x1b[0m")
            .collect::<String>();

        println!(
            "{}\x1b[1;33m{:?}\x1b[0m:'{}'",
            indent,
            rule,
            node.as_span()
                .as_str()
                .lines()
                .map(|line| line.trim())
                .collect::<String>()
        );

        for pair in node.clone().into_inner() {
            recursive_print(Some(&pair), depth + 1);
        }
    }
}
