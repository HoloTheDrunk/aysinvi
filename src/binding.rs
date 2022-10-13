use crate::{
    error::span::Span,
    parsing::{
        AyNode, ComparisonOperator, Expr as PExpr, Multiplier, Node, Statement as PStatement,
    },
};

use {paste::paste, pest::error::LineColLocation, quickscope::ScopeMap};

use std::rc::Rc;

#[derive(PartialEq, Debug, Clone)]
pub struct FunDec {
    name: String,
    args: Vec<String>,
    body: Vec<AyNode<Statement>>,
}

#[derive(PartialEq, Debug, Clone)]
pub struct VarDec {
    names: Vec<String>,
    values: Vec<AyNode<Expr>>,
}

/// A statement is anything that cannot be expected to return a value.
#[derive(PartialEq, Debug, Clone)]
pub enum Statement {
    FunDec(Rc<FunDec>),
    VarDec(Rc<VarDec>),
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
#[derive(PartialEq, Debug, Clone)]
pub enum Expr {
    FunCall {
        def: Rc<FunDec>,
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
    Var(Rc<VarDec>),
    Negated(Box<AyNode<Expr>>),
}
impl Node for Expr {}

pub fn convert(mut ast: &Vec<AyNode<PStatement>>) -> impl Iterator<Item = AyNode<Statement>> + '_ {
    let mut vars = ScopeMap::<String, Rc<VarDec>>::new();
    let mut funs = ScopeMap::<String, Rc<FunDec>>::new();

    ast.iter()
        .map(move |node| convert_statement(node, &mut vars, &mut funs))
}

// This might be retarded lol
macro_rules! convert {
    ($stex:ident $field:ident | $vars:ident $funs:ident) => {
        paste! {
            $field
                .iter()
                .map(|node| [<convert_ $stex>](node, $vars, $funs))
                .collect()
        }
    };
}

fn convert_statement(
    AyNode { span, inner }: &AyNode<PStatement>,
    mut vars: &mut ScopeMap<String, Rc<VarDec>>,
    mut funs: &mut ScopeMap<String, Rc<FunDec>>,
) -> AyNode<Statement> {
    match inner {
        PStatement::VarDec { names, values } => {
            let var_dec = Rc::new(VarDec {
                names: names.clone(),
                values: convert!(expr values | vars funs),
            });

            names
                .iter()
                .for_each(|name| vars.define(name.clone(), var_dec.clone()));

            AyNode {
                span: span.clone(),
                inner: Statement::VarDec(var_dec),
            }
        }
        PStatement::FunDec { name, args, body } => {
            let fun_dec = Rc::new(FunDec {
                name: name.clone(),
                args: args.clone(),
                body: convert!(statement body | vars funs),
            });

            funs.define(name.clone(), fun_dec.clone());

            AyNode {
                span: span.clone(),
                inner: Statement::FunDec(fun_dec),
            }
        }
        PStatement::If {
            cond,
            then,
            otherwise,
        } => AyNode {
            span: span.clone(),
            inner: Statement::If {
                cond: convert_expr(cond, vars, funs),
                then: convert!(statement then | vars funs),
                otherwise: convert!(statement otherwise | vars funs),
            },
        },
        PStatement::Loop { cond, body } => AyNode {
            span: span.clone(),
            inner: Statement::Loop {
                cond: cond.clone().map(|cond| convert_expr(&cond, vars, funs)),
                body: convert!(statement body | vars funs),
            },
        },
        PStatement::Expr(expr) => AyNode {
            span: span.clone(),
            inner: Statement::Expr(convert_expr(expr, vars, funs)),
        },
    }
}

fn convert_expr(
    AyNode { span, inner }: &AyNode<PExpr>,
    mut vars: &mut ScopeMap<String, Rc<VarDec>>,
    mut funs: &mut ScopeMap<String, Rc<FunDec>>,
) -> AyNode<Expr> {
    todo!("{span:?}")
}
