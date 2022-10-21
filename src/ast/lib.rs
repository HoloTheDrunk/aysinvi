use crate::error::span::Span;

use std::str::FromStr;

use {
    paste::paste,
    strum_macros::EnumString
};

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

macro_rules! convert_iter {
    ($stex:ident $field:ident | $($iter:ident)+) => {
        paste::paste! {
            $field
                .iter()
                .map(|node| [<convert_ $stex>](node, $($iter),+))
                .collect::<Result<Vec<AyNode<_>>, Trace>>()
        }
    };

    ($stex:ident $field:ident) => {
        paste::paste! {
            $field
                .iter()
                .map([<convert_ $stex>])
                .collect::<Result<Vec<AyNode<_>>, Trace>>()
        }
    };
}

pub(crate) use convert_iter;
