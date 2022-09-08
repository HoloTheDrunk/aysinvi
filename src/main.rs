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
    let unparsed_file =
        std::fs::read_to_string("examples/negated_redund.ay").expect("Cannot read file");

    match parse(&unparsed_file) {
        Ok(ast) => println!("{:#?}", ast),
        Err(trace) => eprintln!("{}", trace),
    }
}
