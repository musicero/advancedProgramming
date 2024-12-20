# Lab12: Macho interpreter

Macho is a simple concatenative, stack-based, programming language interpreter. The Macho interpreter can be used as interactive CLI, or execute a file containing a program.

I did this lab in rust.

Currently there are 60 tests that pass, but notice that most concepts are covered, I just didn't dedicate time to implement similar features, since it doesn't show much more competence.

## Features I implemented
* Literal values: Integer, Float, Bool, String
* Arithmetic operations: add, sub, mul, div, mod, <, >, ==
* Logic operations: &&, ||, !
* Stack operations: pop, swap, dup
* Control flow: if, for
* Variable Assignment: :=
* Functions: fun, exec
* List operations: map, head, length
* Stdout: print
* Robust type coersion system
* File execution
* Repl mode


# Macho
Macho has two modes:
  - REPL mode: the program reads the input line by line and executes it.
  - Normal mode: the program reads the entire file and executes it.

# Usage
## Run Macho
```bash
# Run Macho in REPL mode
cargo run
# Run Macho in normal mode
cargo run <file.mco>
# check if tests for language passes
cargo test
```
## General info
All statements are separated by space.
A statement can be a literal or an operation.
literals get pushed to the stack, and operations execute on the stack.

## Supported literals:
  - Integer (128bit, signed)
  - Float   (64bit)
  - String  (UTF-8)
  - Bool    

## Supported operations:
  - add, sub, mul, div, mod
  - code_blocks indicated by { } 
  - if else (as "if"), loop, times

## Syntax: Reverse Polish Notation (RPN):
```python
  - 1 1 + => 2
  - 2 2 { + } => 4
  - 2 1 add => 1
  - True { "True" } { "False" } if => "True"
  - True { "True" } "False" if => "True"
  - True "True" "False" if => "True"
  - True False || => True
  - True False && => False
  - 0 { 2 + } 5 times => 10
  - [ 1 2 3 ] { 2 * } map => [ 2 4 6 ]
  - myfunc { 2 * } fun myfunc => CodeBlock(myfunc)
  - 3 myfunc exec => 6
  - myvar 3 := myvar => 3
```

# Under the hood

The compiler consist of two parts:
  - Program Parser: converts the input string into a list of statements
  - Interpreter: executes the list of tokens on the stack

Integers are coerced to floats when operated on with floats. The coercing generally defaults to the most exact type of the two.

# Reflections


Using haskell for this lab would have been a better choice. The pattern matching and the type system would have made the whole thing much more robust and clean.

By using enums for all my types, it became hard to do things generic, which resulted in a lot of code. Several times I only wanted the value from inside the enum, but had to match on the type first, which was really annoying. By using structs, I could have implemented traits and written generic functions for a lot of stuff.
