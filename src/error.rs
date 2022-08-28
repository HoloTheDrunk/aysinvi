use crate::ast::*;

use pest::{
    error::{Error, ErrorVariant, LineColLocation},
    iterators::{Pair, Pairs},
};

#[derive(Debug)]
pub enum Stage {
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
        Trace { stack: vec![(stage, err)] }
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
                    format!(
                       "--> {}\n{}\n |{}\n = {}\n",
                        // Error coordinates
                        match err.line_col {
                            LineColLocation::Pos((y, x)) => format!("{y}:{x}"),
                            LineColLocation::Span((ys, xs), (ye, xe)) =>
                                format!("{ys}:{xs} -> {ye}:{xe}"),
                        },
                        // Line number and line
                        format_args!(
                            " |\n{}| {}",
                            match err.line_col {
                                LineColLocation::Pos((y, _)) => y,
                                LineColLocation::Span((ys, _), _) => ys,
                            },
                            err.line()
                        ),
                        // Underline
                        match err.line_col {
                            LineColLocation::Pos((_, x)) =>
                                format!("{}^", (0..(x)).map(|_| " ").collect::<String>()),
                            LineColLocation::Span((ys, xs), (ye, xe)) =>
                                if ys == ye {
                                    format!(
                                        "{}^{}^",
                                        (0..(xs)).map(|_| " ").collect::<String>(),
                                        (0..(xe - xs - 1)).map(|_| "-").collect::<String>()
                                    )
                                } else {
                                    format!(
                                        "{}^{}",
                                        (0..(xs)).map(|_| " ").collect::<String>(),
                                        (0..(err.line().len() - xs))
                                            .map(|_| "-")
                                            .collect::<String>()
                                    )
                                },
                        },
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
