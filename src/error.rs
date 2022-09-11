use crate::parsing::*;

use pest::{
    error::{Error, ErrorVariant, LineColLocation},
    iterators::{Pair, Pairs},
};

#[derive(Debug)]
pub enum Stage {
    Unknown,
    Parsing,
    AstBuilding,
    Typing,
    Compiling,
}

#[derive(Debug)]
pub struct Trace {
    stack: Vec<(Stage, Error<Rule>)>,
}

impl From<(Stage, Error<Rule>)> for Trace {
    fn from((stage, err): (Stage, Error<Rule>)) -> Self {
        Trace {
            stack: vec![(stage, err)],
        }
    }
}

impl From<Error<Rule>> for Trace {
    fn from(err: Error<Rule>) -> Self {
        Trace {
            stack: vec![(Stage::Parsing, err)],
        }
    }
}

impl From<Box<Error<Rule>>> for Trace {
    fn from(err: Box<Error<Rule>>) -> Self {
        Trace {
            stack: vec![(Stage::Parsing, *err)],
        }
    }
}

impl Trace {
    pub fn new(stage: Stage, err: Error<Rule>) -> Self {
        Trace {
            stack: vec![(stage, err)],
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
                    let line_nbr = match err.line_col {
                        LineColLocation::Pos((y, _)) => y,
                        LineColLocation::Span((ys, _), _) => ys,
                    };

                    let line_nbr_len = line_nbr.to_string().len();

                    let padding = " ".repeat(line_nbr_len);

                    let arrow = format!("{}>", "-".repeat(line_nbr_len));

                    // Error coordinates
                    let coords = match err.line_col {
                        LineColLocation::Pos((y, x)) => format!("{y}:{x}"),
                        LineColLocation::Span((ys, xs), (ye, xe)) => {
                            format!("{ys}:{xs} -> {ye}:{xe}")
                        }
                    };

                    // Underline
                    let underline = match err.line_col {
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
                        err.variant.message()
                    )
                })
                .collect::<String>(),
        )
    }
}

impl Trace {
    pub fn push(&mut self, stage: Stage, err: Error<Rule>) {
        self.stack.push((stage, err))
    }
}
