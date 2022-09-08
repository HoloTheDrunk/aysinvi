# aysìnvi
Lefngapä lì'fya

## What is this?
aysìnvi (no uppercase) is an esolang based on the Na'vi constructed language from Avatar.
The grammar is not fixed yet but some of the main goals of this project are:
 - having the least non-alphabetic symbols possible to represent the oral culture of Na'vi (including math operators),
 - making conjugation of verbs (here functions) have a purpose,
 - building an actually usable standard library that interfaces with lower level APIs to make this esolang technically productive.

## Progress
 - [ ] Grammar
   - [ ] Statements
     - [x] Function definition
       - [x] Infix position marking
       - [x] Arity 0 functions
     - [x] Variable definition
       - [x] Singular variable definition
       - [x] Dual, trial and plural variable definition
     - [ ] Module definition
   - [x] Expressions
     - [x] Literal parsing
       - [x] Numbers (temporary numeric form)
       - [x] Strings
     - [x] Identifiers
     - [x] Function calling
       - [x] Arity =0 `si` form
       - [x] Arity >1 `fa` form
 - [ ] AST building
   - [x] Expressions
     - [x] Numbers (temporary numeric form)
       - [x] Octal parsing
       - [x] Multiplier (`melo`/`pxelo`) parsing
     - [x] Strings
     - [x] Identifiers
     - [x] Function calls
   - [ ] Statements
     - [ ] Function declaration
     - [x] Variable declaration
     - [x] If construct
     - [ ] Loop construct
 - [ ] Interpreter
 - [ ] Compiler
 - [ ] REPL

## Addendum
I have been informed that I am forced to add that since this is made with Rust, it will obviously be 🚀 blazingly fast 🚀.
