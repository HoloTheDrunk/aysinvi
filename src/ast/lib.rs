use crate::error::span::Span;

use std::str::FromStr;

use strum_macros::EnumString;

#[derive(Debug)]
pub enum SourceCode {
    File(String),
    Content(String),
}

/// Node containing a `Span` of code and the corresponding AST
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AyNode<Inner: Node> {
    pub span: Span,
    pub inner: Inner,
}

pub trait Node {}

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
