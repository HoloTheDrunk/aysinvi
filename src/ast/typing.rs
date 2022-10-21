use crate::{
    ast::{
        binding::{Expr as BExpr, Statement as BStatement},
        lib::{AyNode, AyType, ComparisonOperator, Multiplier, Node},
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
    values: Vec<AyNode<TypedExpr>>,
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
    Expr(AyNode<TypedExpr>),
    If {
        cond: AyNode<TypedExpr>,
        then: Vec<AyNode<Statement>>,
        otherwise: Vec<AyNode<Statement>>,
    },
    Loop {
        cond: Option<AyNode<TypedExpr>>,
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
        args: Vec<AyNode<TypedExpr>>,
    },
    Array {
        items: Vec<AyNode<TypedExpr>>,
    },
    Comparison {
        left: Box<AyNode<TypedExpr>>,
        right: Box<AyNode<TypedExpr>>,
        operator: ComparisonOperator,
    },
    Number(i64),
    String(String),
    Var(Rc<VarDec>),
    Negated(Box<AyNode<TypedExpr>>),
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
    todo!();
}

fn convert_expr(AyNode { span, inner }: &AyNode<BExpr>) -> Result<AyNode<TypedExpr>, Trace> {
    todo!();
}
