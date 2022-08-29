use crate::error::*;

use pest::{
    error::{Error, ErrorVariant},
    iterators::{Pair, Pairs},
    Parser,
};

#[derive(Parser)]
#[grammar = "../pest/grammar.pest"]
pub struct AyParser;

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Statement {
    FunDec {
        name: String,
        args: Vec<String>,
        body: Vec<Statement>,
    },
    VarDec {
        names: Vec<String>,
        values: Vec<Expr>,
    },
    Expr(Expr),
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Expr {
    Number(i32),
    Str(String),
}

/// Pushes new error onto stacktrace or returns pred(pair).
fn handle<F, T>(parent: &Pair<Rule>, pair: Pair<Rule>, pred: F) -> Result<T, Trace>
where
    F: FnOnce(Pair<Rule>) -> Result<T, Trace>,
{
    let (span, rule) = (parent.as_span(), parent.as_rule());
    pred(pair).map_err(|mut trace| {
        trace.push(
            Stage::Parsing,
            Error::new_from_span(
                ErrorVariant::ParsingError {
                    positives: vec![rule],
                    negatives: vec![],
                },
                span,
            ),
        );
        trace
    })
}

/// Builds [Statement] from either statement or expr rules
fn dispatch(pair: Pair<Rule>) -> Result<Statement, Trace> {
    match pair.as_rule() {
        Rule::fun_dec | Rule::var_dec => build_ast_from_statement(pair),
        Rule::ident | Rule::string | Rule::number | Rule::call => {
            Ok(Statement::Expr(build_ast_from_expr(pair)?))
        }
        unknown => Err((
            Stage::Compiling,
            Error::new_from_span(
                ErrorVariant::CustomError {
                    message: format!("Unknown rule {:?}", unknown),
                },
                pair.as_span(),
            ),
        )
            .into()),
    }
}

macro_rules! fields {
    ($pair:ident |> $last:ident) => {
        let $last = $pair.next().unwrap();
    };

    ($pair:ident |> $first:ident $(, $tail:ident)*) => {
        let $first = $pair.next().unwrap();
        fields!($pair |> $($tail),*);
    };
}

fn build_ast_from_expr(pair: Pair<Rule>) -> Result<Expr, Trace> {
    Ok(Expr::Number(2))
}

fn build_ast_from_statement(pair: Pair<Rule>) -> Result<Statement, Trace> {
    Ok(Statement::Expr(Expr::Number(2)))
}

pub fn parse(source: &str) -> Result<Vec<Statement>, Error<Rule>> {
    let mut ast = vec![];

    let pairs = AyParser::parse(Rule::program, source)?;
    for pair in pairs {
        dbg!(pair);
    }

    Ok(ast)
}
