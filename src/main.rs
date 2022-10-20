#![allow(unused)]

mod binding;
mod error;
mod highlight;
mod parsing;

extern crate pest;
#[macro_use]
extern crate pest_derive;

use crate::{error::trace::Trace, highlight::highlight_aysinvi, parsing::*};

use pest::{
    error::{Error, ErrorVariant},
    iterators::{Pair, Pairs},
    Parser,
};

fn main() -> Result<(), Trace> {
    macro_rules! print_ast {
        ($ast:ident) => {
            println!("\x1b[1m{}\x1b[0m", stringify!($ast));
            match $ast {
                Ok(ref ast) => println!("{ast:?}"),
                Err(ref trace) => println!("{trace}"),
            };
        };
    }

    let ast = parse(SourceCode::File("./examples/funargs.ay".to_string()));
    print_ast!(ast);

    let bound = binding::convert(&ast?);
    print_ast!(bound);

    println!(
        "{}",
        highlight_aysinvi(
            std::fs::read_to_string("./examples/fizzbuzz.ay")
                .unwrap()
                .as_ref()
        )
    );

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_FOLDER: &str = "./examples/features";

    fn run_tests<F>(path: &str, check: F)
    where
        F: Fn(Result<Vec<AyNode<Statement>>, Trace>) -> bool,
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
