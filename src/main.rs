#![allow(unused)]

mod error;
mod parsing;

extern crate pest;
#[macro_use]
extern crate pest_derive;

use crate::{error::*, parsing::*};

use pest::{
    error::{Error, ErrorVariant},
    iterators::{Pair, Pairs},
    Parser,
};

fn main() {
    let unparsed_file = std::fs::read_to_string("examples/mod.ay").expect("Cannot read file");

    match parse(&unparsed_file) {
        Ok(ast) => println!("{:#?}", ast),
        Err(trace) => eprintln!("{}", trace),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_FOLDER: &str = "examples/features";

    fn run_test<F>(path: &str, check: F)
    where
        F: Fn(Result<Vec<Statement>, Trace>) -> bool,
    {
        let folder = format!("examples/features/{path}");
        let entries = std::fs::read_dir(folder.clone())
            .unwrap_or_else(|err| panic!("{err}: Missing folder `{folder}`"));

        for entry in entries {
            let input = std::fs::read_to_string(entry.unwrap().path().to_str().unwrap()).unwrap();

            assert!(check(parse(&input)));
        }
    }

    #[test]
    fn valid_expressions() {
        run_test("expressions/valid", |output| output.is_ok());
    }

    #[test]
    fn invalid_expressions() {
        run_test("expressions/invalid", |output| output.is_err());
    }
}
