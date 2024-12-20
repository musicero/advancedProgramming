# Lab13: BPROG

* Submission Deadline: **no deadline, portfolio only**
* Programming language: Rust or Haskell 
* Note: Haskell is generally a better choice for this type of tasks, 
  especially with the use of `parsec` modul. Note however, that it is NOT allowed
  to use `parsec` for labs 12 and 13.


# **bprog** :: simple concatenative, stack-based language 

**Note** - the specification below might be unclear or may need amendments. Use Issue tracker to post questions, clarification requests, or modification suggestions.

* Check `rpn-calc` and `rpn-calc-basics` projects for simple parsing and using stack to represent the program and program graph.


# Important

* **DO NOT USE** `parsec` library. All programs **must** be parsed simply by `words`. Submissions with `parsec` **will not** be checked. This is the same as for **Lab12**.
* Report in your specification document all the design decisions and assumptions.
* Precisely report in your self-assessemnt what features are implemented and tested, and what you have not implemented and why.


# Overview

In **Lab12** we have implemented a simple concatenative, stack-based, programming language interpreter. 
The interpreter is called `bprog`. `bprog` accepts instruction from standard input and execute them following the semantics of the language. 
It can be used as interactive CLI, or, it can be fed with a file containing a program and it will execute the file. 
In the CLI mode, the program should parse and interpret the input line by line, in an infinite loop.
In the `input file` mode it should work with the entire input. We will discuss those two modes below.

In **this** lab, **Lab 13**, we follow the same language semantics, and the language specs from Lab12.
However, instead of an interpreter, we would like our `bprog` programs to run with native performance.
That means, we would like to translate it to something that we can compile into native code.


# Approach

There are many approaches to be able to translate and subsequently to compile `a language` into native code. 
Historically, many prototype languages had a translator to C. Then, C was compiled (with GCC) to native code. 
This would be a viable approach, although the modern way to compile 
`bprog` source code, would be to generate LLVM IR directly, or alternatively, to generate IR assembly, which subsequently could be compiled by `llc`
into the intermediate representation and compiled into appropriate backend supported by LLVM.

Note however, that LLVM assembly is quite complex, and it would be too hard to do it from scratch even for simple operations within this course. LLVM assembly represents a steep learning curve. 

Thus, for this lab, we will use WebAssembly text format (WAT). 
The WebAssembly text format (WAT) is much simpler and easier compared to LLVM IR assembly.

WAT: 
* is extremely simple assembly 
* has simple memory model 
* has simple subroutine call conventions
* has simple type system based on integers and floats
* has simple set of operations
* uses stack, that maps nicely into `bprog` semantics. 



# Task difficulty

This is an open ended task. Some transformations are trivial, at the very low level of difficulty. 
Some requiring more insightful planning and thinking with a high level of difficulty.

The simple arithmetic operations like `3 5 +` with the assumption of non-polymorphic literals
` 3 5 ` depicting 32-bit integers addition, translates simply into WAT:

```
i32.const 3
i32.const 5
i32.add
```

As you can see even order of instructions is exactly the same because both, WAT and `bprog` are stack-based. 
This is trivial mapping. However, translating `bprog` operations on lists `[ 1 2 3 ]` or a strings `" Hello World "`
requires more thinking and planning. It will require the use of WAT memory to store and manipulate data.
This may or may not be easy for students.


# Recommendations

* To start, please constrain the work to `i32` integers with simple arithmetic operations. This is trivial.
* Add floating point literals and floating arithmetic. This is also simple. However, it requires more elaborate type system representation and keeping track of types on the AST level. Operations such as `2 1.0 +` require decision what to do with the operations, like `PLUS` - should it be polymorphic? And how to treat literals - should `2` be allowed to be both, integer and floating point type, or should we enforce the use of `2.0` to represent floating point number 2. It is YOUR decision how you want YOUR `bprog` language look like... This is the same as for Lab 12.
* Add Booleans `True` and `False` and add boolean logic operations. Interestingly, WAT does not support bool type, and bools must be represented as `i32` instead, and all logic operations must be implemented using integers. That's already quite a lot of work for supporting simple bool type.


If I were to write everything from scratch myself, I would probably stop here. However, with LLMs helping with the implementation, students might be able to do more. 

Make your own judgement and your own effort. 

The task is open-ended and you can stop at any point. Make your design desitions clear,
and document what works and what does not work. 
Document what you tried but turned out to be too hard or too time consuming to achieve.


Have fun and good luck!
