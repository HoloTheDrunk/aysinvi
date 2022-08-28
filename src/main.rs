#![allow(unused)]

mod error;
mod ast;

extern crate pest;
#[macro_use]
extern crate pest_derive;

use crate::{
    ast::*,
    error::*
};

use pest::{
    error::{Error, ErrorVariant},
    iterators::{Pair, Pairs},
    Parser,
};


fn main() {
    let unparsed_file = std::fs::read_to_string("examples/example.tst")
        .expect("Cannot read tst file");
    let astnode = parse(&unparsed_file).expect("unsuccessful parse");
    println!("{:?}", &astnode);
}
