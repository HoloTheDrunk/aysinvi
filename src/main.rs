#![allow(unused)]

mod binding;
mod error;
mod parsing;

extern crate pest;
#[macro_use]
extern crate pest_derive;

use crate::{error::trace::Trace, parsing::*};

use pest::{
    error::{Error, ErrorVariant},
    iterators::{Pair, Pairs},
    Parser,
};

fn main() {
    match parse(SourceCode::File("./examples/mod.ay".to_string())) {
        Ok(ast) => println!("{:#?}", ast),
        Err(trace) => eprintln!("{}", trace),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_FOLDER: &str = "./examples/features";

    fn run_tests<F>(path: &str, check: F)
    where
        F: Fn(Result<Vec<Statement>, Trace>) -> bool,
    {
        let folder = format!("{TEST_FOLDER}/{path}");
        let mut entries = std::fs::read_dir(folder.clone())
            .unwrap_or_else(|err| panic!("{err}: Missing folder `{folder}`"));

        while let Some(Ok(entry)) = entries.next() {
            let entry = entry.path().to_str().unwrap().to_string();
            eprintln!("Running test {entry}");

            let res = parse(SourceCode::File(entry));
            if let Err(trace) = &res {
                eprintln!("{trace}");
            }

            assert!(check(res));
        }
    }

    #[test]
    fn valid_expressions() {
        run_tests("expressions/valid", |output| output.is_ok());
    }

    #[test]
    fn invalid_expressions() {
        run_tests("expressions/invalid", |output| output.is_err());
    }
}
