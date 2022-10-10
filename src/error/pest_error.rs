use super::{span::Span, trace::TraceError};

use crate::parsing::Rule;

use pest::error::{Error, LineColLocation};

#[derive(Debug)]
pub struct PestError {
    line_col: LineColLocation,
    line: String,
    message: String,
}

impl TraceError for PestError {
    fn from_span(span: Span, message: &str) -> Self
    where
        Self: Sized,
    {
        Self {
            line_col: span.line_col().clone(),
            line: span.as_str().to_owned(),
            message: message.to_owned(),
        }
    }

    fn line_col(&self) -> LineColLocation {
        self.line_col.clone()
    }

    fn line(&self) -> &str {
        self.line.as_ref()
    }

    fn message(&self) -> &str {
        self.message.as_ref()
    }
}

impl From<Error<Rule>> for PestError {
    fn from(err: Error<Rule>) -> Self {
        Self {
            line_col: err.line_col.clone(),
            line: err.line().to_owned(),
            message: err.variant.message().to_string(),
        }
    }
}
