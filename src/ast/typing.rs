use crate::{
    ast::{
        binding::{Expr as PExpr, Statement as PStatement},
        lib::{AyNode, ComparisonOperator, Multiplier, Node},
    },
    error::{
        span::Span,
        trace::{Stage, Trace, TraceError},
        trace_error::Error,
    },
};
