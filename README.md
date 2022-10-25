# aysìnvi
Lefngapä lì'fya

1. [What is this?](#what-is-this?)
2. [Why this name?](#why-this-name?)
3. [Key goals](#key-goals)
4. [Addendum](#addendum)

## What is this?
Aysìnvi is an esolang based on the Na'vi constructed language from Avatar.
Some of the main goals of this project are:
 - having the least non-alphabetic symbols possible to represent the oral culture of Na'vi (including math operators),
 - making conjugation of verbs (here functions) have a purpose,
 - building an actually usable standard library that interfaces with lower level APIs to make this esolang technically productive.

## Why this name?
`ay+` -> **plural** prefix, causes [lenition](https://en.wikipedia.org/wiki/Lenition)  
`tìnvi` -> task, errand, **step** (in an instruction)

## Key goals
 - [x] [Grammar](#grammar)
 - [x] [Parsed AST](#parsed-ast)
 - [x] [Bound AST](#bound-ast) (missing recursion)
 - [ ] [Typed AST](#typed-ast)
 - [ ] [Generic AST pattern replace](#generic-ast-pattern-replace)
 - [ ] Interpreter
 - [ ] Compiler
 - [ ] REPL
 - [ ] LSP

### [Grammar](#progress)
 - [x] Module inclusion
 - [x] Statements
   - [x] If construct
     - [x] Truthy condition
     - [x] Comparison condition 
   - [x] Loop construct
   - [x] Function definition
     - [x] Infix position marking
     - [x] Arity =0 functions
     - [x] Arity >1 functions
   - [x] Variable definition
     - [x] Singular variable definition
     - [x] Dual, trial and plural variable definition
 - [x] Expressions
   - [x] Literal parsing
     - [x] Numbers (temporary numeric form)
     - [x] Strings
   - [x] Data structures
     - [x] Arrays
   - [x] Identifiers
   - [x] Comparisons
   - [x] Function calling
     - [x] Arity =0 `si` form
     - [x] Arity >1 `fa` form

### [Parsed AST](#progress)
 - [x] Module inclusion
 - [x] Statements
   - [x] If construct
   - [x] Loop construct
   - [x] Function declaration
   - [x] Variable declaration
 - [x] Expressions
   - [x] Literals
     - [x] Numbers (temporary numeric form)
       - [x] Octal parsing
       - [x] Multiplier (`melo`/`pxelo`) parsing
     - [x] Strings
   - [x] Data structures
     - [x] Arrays
   - [x] Identifiers
   - [x] Comparisons
   - [x] Function calls

### [Bound AST](#progress)
 - [x] Variables
   - [x] Definition
   - [x] Use
 - [x] Functions 
   - [x] Definition
   - [x] Call
   - [x] Recursion

### [Typed AST](#progress)
 - [ ] Definitions
   - [ ] Variables
   - [ ] Functions
 - [ ] Expressions
   - [ ] Literals
   - [ ] Variable use
   - [ ] Function call
   - [ ] Comparisons

### [Generic AST pattern replace](#progress)
This step aims to provide a nice API to enable advanced users to consisely define their own mini pattern finding language.  
This must be generic and user-defined as modules use different types of structs and enums that rarely have the same names or components.  
The general structure should be the same.  
For example:
 - `Fd(_,_,[Vd(_, _)])/d` could be a representation for  
 "Match any **F**unction **d**efinition with a single **V**ariable **d**efinition in its body and **d**elete its subtree from the AST".
 - `[Fd(name,argnames,body),Fc(name,args)]/s/[Vd(argnames,args),body]` could be a representation for  
 "Match any **F**unction **d**efinition followed only by a **F**unction **c**all to it and **s**ubstitute this match with a 
 **V**ariable **d**efinition of the call **arg**(s)uments followed by the function's **body**".

## Addendum
I have been informed that I am forced to add that since this is made with Rust, it will obviously be 🚀 blazingly fast 🚀
