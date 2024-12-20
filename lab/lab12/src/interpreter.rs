use crate::tokenizer::*;

use std::collections::HashMap;

#[derive(Debug)]
pub enum ProgramError {
    NotAFunction,
    MultipleValuesInMap,
    ExpectedSymbol,
    ExpectedList,
    ListEmpty,
    ExpectedCodeBlock,
    TypeError,
    StackEmpty,
    ExpectedLiteral,
    ExpectedBool,
    TypeMismatch,
}

pub enum LiteralPair {
    Int(i128, i128),
    Float(f64, f64),
    Bool(bool, bool),
    String(String, String),
}

pub struct Runtime {
    stack: Vec<Exp>,
    variables: HashMap<String, Exp>,
    functions: HashMap<String, Exp>,
}

#[allow(dead_code)]
impl Runtime {
    pub fn new() -> Runtime {
        Runtime {
            stack: Vec::new(),
            variables: HashMap::new(),
            functions: HashMap::new(),
        }
    }
    pub fn from(
        stack: Vec<Exp>,
        variables: HashMap<String, Exp>,
        functions: HashMap<String, Exp>,
    ) -> Runtime {
        Runtime {
            stack,
            variables,
            functions,
        }
    }
    pub fn stack_len(&self) -> usize {
        self.stack.len()
    }
    pub fn push(&mut self, expression: Exp) {
        self.stack.push(expression)
    }
    pub fn pop(&mut self) -> Option<Exp> {
        self.stack.pop()
    }

    pub fn pop_check(&mut self) -> Result<Exp, ProgramError> {
        let exp = self.pop().ok_or(ProgramError::StackEmpty)?;
        Ok(exp)
    }
    pub fn pop_symbol(&mut self) -> Result<String, ProgramError> {
        match self.pop().ok_or(ProgramError::StackEmpty)? {
            Exp::Symbol(name) => Ok(name),
            _ => Err(ProgramError::ExpectedSymbol),
        }
    }
    pub fn pop_literal(&mut self) -> Result<Literal, ProgramError> {
        match self.pop_check()? {
            Exp::Literal(literal) => Ok(literal),
            _ => Err(ProgramError::ExpectedLiteral),
        }
    }
    pub fn print_stack(&self) {
        print!("Stack: ");
        for exp in self.stack.iter() {
            print!("{:?} ", exp);
        }
        println!();
    }
    pub fn pop_list(&mut self) -> Result<Vec<Exp>, ProgramError> {
        match self.pop_check()? {
            Exp::List(list) => Ok(list),
            _ => Err(ProgramError::ExpectedList),
        }
    }

    pub fn pop_coerce(&mut self) -> Result<LiteralPair, ProgramError> {
        let b = self.pop_literal()?;
        let a = self.pop_literal()?;
        coerce_literals(a, b)
    }

    pub fn pop_int(&mut self) -> Result<i128, ProgramError> {
        match self.pop_literal()? {
            Literal::Int(value) => Ok(value),
            _ => Err(ProgramError::ExpectedLiteral),
        }
    }

    pub fn pop_bool(&mut self) -> Result<bool, ProgramError> {
        match self.pop_literal()? {
            Literal::Bool(value) => Ok(value),
            _ => Err(ProgramError::ExpectedBool),
        }
    }

    pub fn pop_code_block(&mut self) -> Result<Vec<Token>, ProgramError> {
        match self.pop_check()? {
            Exp::CodeBlock(tokens) => Ok(tokens),
            _ => Err(ProgramError::ExpectedCodeBlock),
        }
    }

    pub fn pop_literal_pair(&mut self) -> Result<(Literal, Literal), ProgramError> {
        let b = self.pop_literal()?;
        let a = self.pop_literal()?;
        // check if literals is same type
        let pair = (a, b);
        match pair {
            (Literal::Int(_), Literal::Int(_)) => Ok(pair),
            (Literal::Bool(_), Literal::Bool(_)) => Ok(pair),
            (Literal::Float(_), Literal::Float(_)) => Ok(pair),
            (Literal::String(_), Literal::String(_)) => Ok(pair),
            _ => Err(ProgramError::TypeMismatch),
        }
    }
}

pub fn coerce_literals(a: Literal, b: Literal) -> Result<LiteralPair, ProgramError> {
    Ok(match (a, b) {
        (Literal::Int(a), Literal::Int(b)) => LiteralPair::Int(a, b),
        (Literal::Int(a), Literal::Float(b)) => LiteralPair::Float(a as f64, b),
        (Literal::Int(a), Literal::Bool(b)) => LiteralPair::Int(a, b as i128),
        (Literal::Int(_), Literal::String(_)) => return Err(ProgramError::TypeMismatch),

        (Literal::Float(a), Literal::Int(b)) => LiteralPair::Float(a, b as f64),
        (Literal::Float(a), Literal::Float(b)) => LiteralPair::Float(a, b),
        (Literal::Float(a), Literal::Bool(b)) => LiteralPair::Float(a, b as i64 as f64),
        (Literal::Float(_), Literal::String(_)) => return Err(ProgramError::TypeMismatch),

        (Literal::Bool(a), Literal::Int(b)) => LiteralPair::Int(a as i128, b),
        (Literal::Bool(a), Literal::Float(b)) => LiteralPair::Float(a as i64 as f64, b),
        (Literal::Bool(a), Literal::Bool(b)) => LiteralPair::Bool(a, b),
        (Literal::Bool(_), Literal::String(_)) => return Err(ProgramError::TypeMismatch),

        (Literal::String(a), Literal::String(b)) => LiteralPair::String(a, b),
        (Literal::String(_), _) => return Err(ProgramError::TypeMismatch),
    })
}

pub fn interpret(runtime: &mut Runtime, token: Token) -> Result<(), ProgramError> {
    match token {
        Token::Exp(expression) => interpret_expression(runtime, expression),
        Token::Op(operation) => interpret_operation(runtime, operation),
    }
}

fn interpret_expression(runtime: &mut Runtime, expression: Exp) -> Result<(), ProgramError> {
    match expression {
        Exp::Symbol(symbol) => interpret_symbol(runtime, symbol),
        _ => Ok(runtime.push(expression)),
    }
}

fn interpret_symbol(runtime: &mut Runtime, symbol: String) -> Result<(), ProgramError> {
    if let Some(exp) = runtime.variables.get(&symbol) {
        runtime.push(exp.clone());
    } else if let Some(exp) = runtime.functions.get(&symbol) {
        match exp {
            Exp::CodeBlock(block) => execute_block(runtime, block.clone())?,
            _ => Err(ProgramError::NotAFunction)?,
        };
    } else {
        runtime.push(Exp::Symbol(symbol));
    }
    Ok(())
}

fn interpret_operation(runtime: &mut Runtime, operation: Op) -> Result<(), ProgramError> {
    match operation {
        // Op::Arithmetic(operationn) => interpret_arithmetic(runtime, operation),
        Op::Arithmetic(operation) => {
            let result = Ok(match runtime.pop_coerce()? {
                LiteralPair::Int(a, b) => Literal::Int(match operation {
                    Arithmetic::Add => a + b,
                    Arithmetic::Sub => a - b,
                    Arithmetic::Mul => a * b,
                    Arithmetic::Div => a / b,
                    Arithmetic::Mod => a % b,
                }),
                LiteralPair::Float(a, b) => Literal::Float(match operation {
                    Arithmetic::Add => a + b,
                    Arithmetic::Sub => a - b,
                    Arithmetic::Mul => a * b,
                    Arithmetic::Div => a / b,
                    Arithmetic::Mod => a % b,
                }),
                LiteralPair::Bool(a, b) => Literal::Bool(match operation {
                    Arithmetic::Add => a || b,
                    Arithmetic::Sub => a & !b,
                    Arithmetic::Mul => a & b,
                    Arithmetic::Mod => return Err(ProgramError::TypeError),
                    Arithmetic::Div => return Err(ProgramError::TypeError),
                }),
                LiteralPair::String(a, b) => Literal::String(match operation {
                    Arithmetic::Add => format!("{}{}", a, b),
                    _ => return Err(ProgramError::TypeError),
                }),
            });
            runtime.push(Exp::Literal(result?));
        }
        Op::Logical(operation) => {
            let result = match operation {
                Logical::Not => {
                    let a = runtime.pop_bool()?;
                    !a
                }
                Logical::And => {
                    let a = runtime.pop_bool()?;
                    let b = runtime.pop_bool()?;
                    a && b
                }
                Logical::Or => {
                    let a = runtime.pop_bool()?;
                    let b = runtime.pop_bool()?;
                    a || b
                }
            };
            runtime.push(Exp::Literal(Literal::Bool(result)))
        }
        Op::Comparison(operation) => {
            let pair = runtime.pop_coerce()?;
            let result = match operation {
                Comparison::Equal => match pair {
                    LiteralPair::Int(a, b) => a == b,
                    LiteralPair::Float(a, b) => a == b,
                    LiteralPair::Bool(a, b) => a == b,
                    LiteralPair::String(a, b) => a == b,
                },
                Comparison::Less => match pair {
                    LiteralPair::Int(a, b) => a < b,
                    LiteralPair::Float(a, b) => a < b,
                    LiteralPair::Bool(a, b) => a & b,
                    LiteralPair::String(a, b) => a < b,
                },
                Comparison::Greater => match pair {
                    LiteralPair::Int(a, b) => a > b,
                    LiteralPair::Float(a, b) => a > b,
                    LiteralPair::Bool(a, b) => a & !b,
                    LiteralPair::String(a, b) => a > b,
                },
            };
            runtime.push(Exp::Literal(Literal::Bool(result)));
        }
        Op::StackOp(operation) => match operation {
            StackOp::Pop => {
                runtime.pop();
            }
            StackOp::Swap => {
                let a = runtime.pop_check()?;
                let b = runtime.pop_check()?;
                runtime.push(a);
                runtime.push(b);
            }
            StackOp::Dup => {
                let a = runtime.pop_check()?;
                runtime.push(a.clone());
                runtime.push(a);
            }
        },
        Op::FlowOp(operation) => match operation {
            FlowOp::Exec => {
                let block = runtime.pop_code_block()?;
                // give acces to outer stack for yum yum tricks
                execute_block(runtime, block)?;
            }
            FlowOp::If => {
                let false_literal = runtime.pop_check()?;
                let true_literal = runtime.pop_check()?;
                let cond = runtime.pop_bool()?;

                let literal = match cond {
                    true => true_literal,
                    false => false_literal,
                };

                match literal {
                    Exp::CodeBlock(block) => execute_block(runtime, block)?,
                    Exp::Literal(_) | Exp::List(_) | Exp::Symbol(_) => runtime.push(literal),
                }
            }
            FlowOp::For => {
                let n = runtime.pop_int()?;
                let block = runtime.pop_code_block()?;
                for _ in 0..n {
                    execute_block(runtime, block.clone())?;
                }
            }
        },
        Op::VarOp(operation) => match operation {
            VarOp::AssignVar => {
                // allowing for anonymous functions to be assigned to variables
                let exp = runtime.pop_check()?;
                let var_name = runtime.pop_symbol()?;
                // Disallowing code blocks as variables
                if let Exp::CodeBlock(_) = exp {
                    Err(ProgramError::TypeError)?
                };
                runtime.variables.insert(var_name, exp);
            }
            VarOp::AssignFun => {
                let block = runtime.pop_code_block()?;
                let var_name = runtime.pop_symbol()?;
                // Could also have put codeblocks directly as variables, which would
                // result in functions being first class citizens like in Rust!
                runtime.functions.insert(var_name, Exp::CodeBlock(block));
            }
        },
        Op::ListOp(operation) => match operation {
            ListOp::Head => {
                let list = runtime.pop_list()?;
                let head = list.first().ok_or(ProgramError::ListEmpty)?.clone();
                runtime.push(head);
            }
            ListOp::Length => {
                let list = runtime.pop_list()?;
                runtime.push(Exp::Literal(Literal::Int(list.len() as i128)));
            }
            ListOp::Map => {
                let block = runtime.pop_code_block()?;
                let list = runtime.pop_list()?;
                let mut mapped_list = Vec::new();

                for exp in list {
                    // create a new runtime for the block to operate on elm, optimally it should
                    // take a mutable reference to current runtimes variables
                    let mut elm_runtime = Runtime::from(
                        Vec::new(),
                        runtime.variables.clone(),
                        runtime.functions.clone(),
                    );
                    elm_runtime.push(exp); // push current elm to stack
                                           // execute the block on the temporary elm_runtime
                    execute_block(&mut elm_runtime, block.clone())?;
                    match elm_runtime.stack_len() {
                        0 => (),
                        1 => mapped_list.push(elm_runtime.pop_check()?),
                        _ => return Err(ProgramError::MultipleValuesInMap),
                    }
                }
                runtime.push(Exp::List(mapped_list));
            }
        },
        Op::IO(operation) => match operation {
            IO::Print => {
                let exp = runtime.pop_check()?;
                println!("{}", exp);
            }
        },
    }
    Ok(())
}

fn execute_block(runtime: &mut Runtime, tokens: Vec<Token>) -> Result<(), ProgramError> {
    for token in tokens {
        interpret(runtime, token)?;
    }
    Ok(())
}
