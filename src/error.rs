use crate::parsing::*;

use pest::{
    error::{Error, ErrorVariant, LineColLocation},
    iterators::{Pair, Pairs},
    Span,
};

use std::fmt::Debug;

#[derive(Debug)]
pub enum Stage {
    Unknown,
    Parsing,
    AstBuilding,
    Typing,
    Compiling,
}

#[derive(Debug, Default)]
pub struct Trace {
    stack: Vec<(Stage, Box<dyn TraceError>)>,
}

pub trait TraceError: Debug {
    fn from_span(span: Span, message: &str) -> Self
    where
        Self: Sized;
    fn line_col(&self) -> LineColLocation;
    fn line(&self) -> &str;
    fn message(&self) -> &str;
}

impl<T: TraceError + 'static> From<(Stage, T)> for Trace {
    fn from((stage, err): (Stage, T)) -> Self {
        Trace {
            stack: vec![(stage, Box::new(err))],
        }
    }
}

impl<T: TraceError + 'static> From<T> for Trace {
    fn from(err: T) -> Self {
        Trace {
            stack: vec![(Stage::Parsing, Box::new(err))],
        }
    }
}

impl Trace {
    pub fn new<T: TraceError + 'static>(stage: Stage, err: T) -> Self {
        Trace {
            stack: vec![(stage, Box::new(err))],
        }
    }

    pub fn new_from_message(stage: Stage, pair: &Pair<Rule>, message: String) -> Self {
        let mut res = Trace::default();
        res.push_message(stage, pair, message);
        res
    }

    pub fn push<T: TraceError + 'static>(&mut self, stage: Stage, err: T) {
        self.stack.push((stage, Box::new(err)))
    }

    pub fn push_message(&mut self, stage: Stage, pair: &Pair<Rule>, message: String) {
        self.stack.push((
            stage,
            Box::new(PestError::from_span(pair.as_span(), message.as_ref())),
        ))
    }
}

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
            line_col: LineColLocation::Span(span.start_pos().line_col(), span.end_pos().line_col()),
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

impl std::fmt::Display for Trace {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Deepest error first\n{}",
            self.stack
                .iter()
                .map(|(stage, err)| {
                    let line_nbr = match err.line_col() {
                        LineColLocation::Pos((y, _)) => y,
                        LineColLocation::Span((ys, _), _) => ys,
                    };

                    let line_nbr_len = line_nbr.to_string().len();

                    let padding = " ".repeat(line_nbr_len);

                    let arrow = format!("{}>", "-".repeat(line_nbr_len));

                    // Error coordinates
                    let coords = match err.line_col() {
                        LineColLocation::Pos((y, x)) => format!("{y}:{x}"),
                        LineColLocation::Span((ys, xs), (ye, xe)) => {
                            format!("{ys}:{xs} -> {ye}:{xe}")
                        }
                    };

                    // Underline
                    let underline = match err.line_col() {
                        LineColLocation::Pos((_, x)) => format!("{}^", " ".repeat(x)),
                        LineColLocation::Span((ys, xs), (ye, xe)) => {
                            if ys == ye {
                                format!("{}^{}^", " ".repeat(xs), "-".repeat(xe - xs - 1))
                            } else {
                                format!("{}^{}", " ".repeat(xs), "-".repeat(err.line().len() - xs))
                            }
                        }
                    };

                    // ---> STAGE | COORDS
                    //    |
                    // NBR| LINE
                    //    | UNDERLINE
                    //    = ERROR
                    format!(
                        "{arrow} {stage:?} | {coords}\n\
                         {padding}|\n\
                         {}\n\
                         {padding}|{underline}\n\
                         {padding}= {}\n",
                        // Line number and line
                        format_args!("{}| {}", line_nbr, err.line()),
                        // Error
                        err.message()
                    )
                })
                .collect::<String>(),
        )
    }
}
