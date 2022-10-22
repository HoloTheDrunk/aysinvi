use crate::{
    ast::{
        binding::{Expr as BExpr, Statement as BStatement},
        lib::{convert_iter, AyNode, AyType, ComparisonOperator, Multiplier, Node},
    },
    error::{
        span::Span,
        trace::{Stage, Trace, TraceError},
        trace_error::Error,
    },
};

use std::rc::Rc;

use paste::paste;

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Tense {
    Present,
    Imminent,
    Future,
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct FunDec {
    name: String,
    args: Vec<Rc<VarDec>>,
    body: Vec<AyNode<Statement>>,
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct VarDec {
    names: Vec<String>,
    values: Vec<TypedExpr>,
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct TypedExpr {
    expr_type: AyType,
    inner: Expr,
}
impl Node for TypedExpr {}

/// A statement is anything that cannot be expected to return a value.
#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Statement {
    FunDec(Rc<FunDec>),
    VarDec(Rc<VarDec>),
    Expr(TypedExpr),
    If {
        cond: TypedExpr,
        then: Vec<AyNode<Statement>>,
        otherwise: Vec<AyNode<Statement>>,
    },
    Loop {
        cond: Option<TypedExpr>,
        body: Vec<AyNode<Statement>>,
    },
}
impl Node for Statement {}

/// An expression is anything that is or returns a value.
#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Expr {
    FunCall {
        tense: Tense,
        dec: Rc<FunDec>,
        name: String,
        args: Vec<TypedExpr>,
    },
    Array {
        items: Vec<TypedExpr>,
    },
    Comparison {
        left: Box<TypedExpr>,
        right: Box<TypedExpr>,
        operator: ComparisonOperator,
    },
    Number(i64),
    String(String),
    Var(Rc<VarDec>),
    Negated(Box<TypedExpr>),
}
impl Node for Expr {}

pub fn convert(mut ast: &Vec<AyNode<BStatement>>) -> Result<Vec<AyNode<Statement>>, Trace> {
    ast.iter()
        .map(convert_statement)
        .collect::<Result<Vec<AyNode<Statement>>, Trace>>()
}

fn convert_statement(
    AyNode { span, inner }: &AyNode<BStatement>,
) -> Result<AyNode<Statement>, Trace> {
    match inner {
        BStatement::Expr(expr) => Ok(AyNode {
            span: span.clone(),
            inner: Statement::Expr(convert_expr(expr)?),
        }),
        _ => todo!(),
    }
}

fn convert_expr(AyNode { span, inner }: &AyNode<BExpr>) -> Result<TypedExpr, Trace> {
    match inner {
        BExpr::Number(number) => Ok(TypedExpr {
            expr_type: AyType::Number,
            inner: Expr::Number(*number),
        }),
        BExpr::String(string) => Ok(TypedExpr {
            expr_type: AyType::String,
            inner: Expr::String(string.clone()),
        }),
        BExpr::Array { items } => {
            let items = convert_iter!(expr items)?;

            Ok(TypedExpr {
                expr_type: AyType::Array(Box::new(
                    items
                        .get(0)
                        .ok_or_else(|| {
                            Trace::new(
                                Stage::Typing,
                                Error::from_span(span.clone(), "Cannot defined empty arrays"),
                            )
                        })?
                        .expr_type
                        .clone(),
                )),
                inner: Expr::Array {
                    items: items.clone(),
                },
            })
        }
        BExpr::Negated(expr) => {
            let expr = convert_expr(expr)?;

            Ok(TypedExpr {
                expr_type: match expr.expr_type {
                    AyType::Number => Ok(AyType::Number),
                    AyType::Array(_) => Ok(AyType::Bool),
                    _ => Err(Trace::new(
                        Stage::Binding,
                        Error::from_span(
                            span.clone(),
                            format!("Can only negate Number or Array, not {:?}", expr.expr_type)
                                .as_ref(),
                        ),
                    )),
                }?,
                inner: todo!(),
            })
        }
        BExpr::Comparison {
            left,
            right,
            operator,
        } => {
            let left = convert_expr(left)?;
            let right = convert_expr(right)?;

            if left.expr_type == right.expr_type {
                Ok(TypedExpr {
                    expr_type: AyType::Bool,
                    inner: Expr::Comparison {
                        left: Box::new(left),
                        right: Box::new(right),
                        operator: operator.clone(),
                    },
                })
            } else {
                Err(Trace::new(
                    Stage::Binding,
                    Error::from_span(
                        span.clone(),
                        format!(
                            "Cannot compare {:?} and {:?}",
                            left.expr_type, right.expr_type
                        )
                        .as_ref(),
                    ),
                ))
            }
        }
        _ => todo!(),
    }
}
