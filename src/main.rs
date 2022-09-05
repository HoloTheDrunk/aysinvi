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
    let unparsed_file = std::fs::read_to_string("examples/example.ay").expect("Cannot read file");

    let astnode = parse(&unparsed_file).expect("unsuccessful parse");
    println!("{:?}", &astnode);
}
