use crate::{
    ast::{
        lib::{convert_iter, AyNode, ComparisonOperator, Multiplier, Node},
        parsing::{Expr as PExpr, Statement as PStatement},
    },
    error::{
        span::Span,
        trace::{Stage, Trace, TraceError},
        trace_error::Error,
    },
};

use {pest::error::LineColLocation, quickscope::ScopeMap};

#[derive(PartialEq, Eq, Default, Debug, Clone)]
pub struct FunDec {
    name: String,
    args: Vec<String>,
    body: Vec<AyNode<Statement>>,
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Tense {
    Present,
    Imminent,
    Future,
}

#[derive(PartialEq, Eq, Debug, Default, Clone)]
pub struct VarDec {
    names: Vec<String>,
    values: Vec<AyNode<Expr>>,
}

/// A statement is anything that cannot be expected to return a value.
#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Statement {
    FunDec(FunDec),
    VarDec(VarDec),
    Expr(AyNode<Expr>),
    If {
        cond: AyNode<Expr>,
        then: Vec<AyNode<Statement>>,
        otherwise: Vec<AyNode<Statement>>,
    },
    Loop {
        cond: Option<AyNode<Expr>>,
        body: Vec<AyNode<Statement>>,
    },
}
impl Node for Statement {}

/// An expression is anything that is or returns a value.
#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Expr {
    FunCall {
        tense: Tense,
        name: String,
        args: Vec<AyNode<Expr>>,
    },
    Array {
        items: Vec<AyNode<Expr>>,
    },
    Comparison {
        left: Box<AyNode<Expr>>,
        right: Box<AyNode<Expr>>,
        operator: ComparisonOperator,
    },
    Number(i64),
    String(String),
    Var(String),
    Negated(Box<AyNode<Expr>>),
}
impl Node for Expr {}

pub fn convert(mut ast: &Vec<AyNode<PStatement>>) -> Result<Vec<AyNode<Statement>>, Trace> {
    let mut vars = ScopeMap::<String, ()>::new();
    let mut funs = ScopeMap::<String, ()>::new();

    ast.iter()
        .map(move |node| convert_statement(node, &mut vars, &mut funs))
        .collect::<Result<Vec<AyNode<Statement>>, Trace>>()
}

macro_rules! wrap_scope {
    ($($scoped_map:ident),* | $actions:block) => {
        {
            $( $scoped_map.push_layer(); )*

            let res = $actions;

            $( $scoped_map.pop_layer(); )*

            res
        }
    };
}

fn convert_statement(
    AyNode { span, inner }: &AyNode<PStatement>,
    mut vars: &mut ScopeMap<String, ()>,
    mut funs: &mut ScopeMap<String, ()>,
) -> Result<AyNode<Statement>, Trace> {
    match inner {
        PStatement::VarDec { names, values } => {
            names.iter().for_each(|name| vars.define(name.clone(), ()));

            Ok(AyNode {
                span: span.clone(),
                inner: Statement::VarDec(VarDec {
                    names: names.clone(),
                    values: convert_iter!(expr values | vars funs)?,
                }),
            })
        }
        PStatement::FunDec { name, args, body } => {
            funs.define(name.clone(), ());
            Ok(AyNode {
                span: span.clone(),
                inner: Statement::FunDec(FunDec {
                    name: name.clone(),
                    args: args.clone(),
                    body: wrap_scope!(
                        vars,
                        funs | {
                            args.iter().for_each(|var| vars.define(var.clone(), ()));
                            convert_iter!(statement body | vars funs)?
                        }
                    ),
                }),
            })
        }
        PStatement::If {
            cond,
            then,
            otherwise,
        } => Ok(AyNode {
            span: span.clone(),
            inner: Statement::If {
                cond: convert_expr(cond, vars, funs)?,
                then: wrap_scope!(vars, funs | { convert_iter!(statement then | vars funs)? }),
                otherwise: wrap_scope!(
                    vars,
                    funs | { convert_iter!(statement otherwise | vars funs)? }
                ),
            },
        }),
        PStatement::Loop { cond, body } => Ok(AyNode {
            span: span.clone(),
            inner: Statement::Loop {
                cond: cond
                    .clone()
                    .map(|cond| convert_expr(&cond, vars, funs))
                    .transpose()?,
                body: wrap_scope!(vars, funs | { convert_iter!(statement body | vars funs)? }),
            },
        }),
        PStatement::Expr(expr) => Ok(AyNode {
            span: span.clone(),
            inner: Statement::Expr(convert_expr(expr, vars, funs)?),
        }),
    }
}

fn convert_expr(
    AyNode { span, inner }: &AyNode<PExpr>,
    mut vars: &mut ScopeMap<String, ()>,
    mut funs: &mut ScopeMap<String, ()>,
) -> Result<AyNode<Expr>, Trace> {
    match inner {
        PExpr::Ident(name) => {
            if let Some(rc) = vars.get(name) {
                Ok(AyNode {
                    span: span.clone(),
                    inner: Expr::Var(name.clone()),
                })
            } else {
                Err(Trace::new(
                    Stage::Binding,
                    Error::from_span(
                        span.clone(),
                        format!("Undefined variable: '{name}'{}", closest(vars, name)).as_ref(),
                    ),
                ))
            }
        }
        PExpr::FunCall { name, args } => match match_function(name, funs) {
            Some(tense) => Ok(AyNode {
                span: span.clone(),
                inner: Expr::FunCall {
                    tense,
                    name: name.clone(),
                    args: convert_iter!(expr args | vars funs)?,
                },
            }),
            None => Err(Trace::new(
                Stage::Binding,
                Error::from_span(
                    span.clone(),
                    format!("Undefined function: '{name}'{}", closest(funs, name)).as_ref(),
                ),
            )),
        },
        PExpr::Number(num) => Ok(AyNode {
            span: span.clone(),
            inner: Expr::Number(*num),
        }),
        PExpr::String(string) => Ok(AyNode {
            span: span.clone(),
            inner: Expr::String(string.clone()),
        }),
        PExpr::Negated(expr) => Ok(AyNode {
            span: span.clone(),
            inner: Expr::Negated(Box::new(convert_expr(expr, vars, funs)?)),
        }),
        PExpr::Comparison {
            left,
            right,
            operator,
        } => Ok(AyNode {
            span: span.clone(),
            inner: Expr::Comparison {
                left: Box::new(convert_expr(left, vars, funs)?),
                right: Box::new(convert_expr(right, vars, funs)?),
                operator: operator.clone(),
            },
        }),
        PExpr::Array { items } => Ok(AyNode {
            span: span.clone(),
            inner: Expr::Array {
                items: convert_iter!(expr items | vars funs)?,
            },
        }),
    }
}

fn closest<T>(scope_map: &ScopeMap<String, T>, name: &str) -> String {
    scope_map
        .keys()
        .map(|key| (key, distance::levenshtein(name, key)))
        .min_by(|(_, d1), (_, d2)| usize::cmp(d1, d2))
        .filter(|(key, dist)| *dist * 2 < key.len())
        .map(|(key, _)| format!(". Maybe you meant: '{}'?", key.replace('.', "")))
        .unwrap_or_else(|| "".to_owned())
}

fn match_function(name: &str, funs: &ScopeMap<String, ()>) -> Option<Tense> {
    funs.iter().find_map(|(key, fun)| {
        if key.contains('.') {
            let (left, right) = key.split_once('.').unwrap();

            if format!("{left}{right}") == name {
                Some(Tense::Present)
            } else if format!("{left}ìy{right}") == name {
                Some(Tense::Imminent)
            } else if format!("{left}ay{right}") == name {
                Some(Tense::Future)
            } else {
                None
            }
        } else {
            (key == name).then(|| Tense::Present)
        }
    })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    /// Test:
    /// ````
    /// fn scope {
    ///    fn t.aron {}
    ///    taron()   // valid
    ///    tìyaron() // valid
    ///    tayaron() // valid
    /// }
    /// taron() -- invalid
    /// ````
    fn test_match_function() {
        let mut funs = ScopeMap::<String, ()>::new();

        wrap_scope!(
            funs | {
                funs.define("scope".to_owned(), ());

                wrap_scope!(
                    funs | {
                        funs.define("t.aron".to_owned(), ());

                        vec!["taron", "tìyaron", "tayaron"]
                            .iter()
                            .map(|name| (name, match_function(name, &funs)))
                            .for_each(|(name, res)| {
                                assert!(res.is_some(), "Function not found: '{}'", name)
                            });
                    }
                );

                assert!(match_function("taron", &funs).is_none());
            }
        );
    }
}
