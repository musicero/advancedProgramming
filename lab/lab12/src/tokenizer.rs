pub mod token_type;
pub use token_type::*;

#[derive(Debug)]
pub enum ParserError {
    StringNotClosed,
    ArrayNotClosed,
    CodeBlockNotClosed,
    OperationInArray,
    ProgramEnd,
}

pub fn tokenize(input: String) -> Result<Vec<Token>, ParserError> {
    let mut tokens: Vec<&str> = input.trim().split_whitespace().rev().collect();
    let mut program_tokens: Vec<Token> = Vec::new();
    while !tokens.is_empty() {
        program_tokens.push(tokenize_next(&mut tokens)?);
    }

    Ok(program_tokens)
}

fn tokenize_next(tokens: &mut Vec<&str>) -> Result<Token, ParserError> {
    let word = tokens.pop().ok_or(ParserError::ProgramEnd)?;
    Ok(match word {
        "True" => Token::Exp(Exp::Literal(Literal::Bool(true))),
        "False" => Token::Exp(Exp::Literal(Literal::Bool(false))),
        "{" => Token::Exp(Exp::CodeBlock(parse_code_block(tokens)?)),
        "[" => Token::Exp(Exp::List(parse_array(tokens)?)),
        "\"" => Token::Exp(Exp::Literal(Literal::String(parse_string(tokens)?))),
        "+" => Token::Op(Op::Arithmetic(Arithmetic::Add)),
        "-" => Token::Op(Op::Arithmetic(Arithmetic::Sub)),
        "*" => Token::Op(Op::Arithmetic(Arithmetic::Mul)),
        "/" => Token::Op(Op::Arithmetic(Arithmetic::Div)),
        "div" => Token::Op(Op::Arithmetic(Arithmetic::Div)),
        "mul" => Token::Op(Op::Arithmetic(Arithmetic::Mul)),
        "add" => Token::Op(Op::Arithmetic(Arithmetic::Add)),
        "sub" => Token::Op(Op::Arithmetic(Arithmetic::Sub)),
        "mod" => Token::Op(Op::Arithmetic(Arithmetic::Mod)),
        "%" => Token::Op(Op::Arithmetic(Arithmetic::Mod)),
        "&&" => Token::Op(Op::Logical(Logical::And)),
        "||" => Token::Op(Op::Logical(Logical::Or)),
        "!" => Token::Op(Op::Logical(Logical::Not)),
        "and" => Token::Op(Op::Logical(Logical::And)),
        "or" => Token::Op(Op::Logical(Logical::Or)),
        "not" => Token::Op(Op::Logical(Logical::Not)),
        "==" => Token::Op(Op::Comparison(Comparison::Equal)),
        "<" => Token::Op(Op::Comparison(Comparison::Less)),
        ">" => Token::Op(Op::Comparison(Comparison::Greater)),
        "pop" => Token::Op(Op::StackOp(StackOp::Pop)),
        "swap" => Token::Op(Op::StackOp(StackOp::Swap)),
        "dup" => Token::Op(Op::StackOp(StackOp::Dup)),
        "exec" => Token::Op(Op::FlowOp(FlowOp::Exec)),
        "if" => Token::Op(Op::FlowOp(FlowOp::If)),
        "for" => Token::Op(Op::FlowOp(FlowOp::For)),
        ":=" => Token::Op(Op::VarOp(VarOp::AssignVar)),
        "fun" => Token::Op(Op::VarOp(VarOp::AssignFun)),
        "head" => Token::Op(Op::ListOp(ListOp::Head)),
        "length" => Token::Op(Op::ListOp(ListOp::Length)),
        "map" => Token::Op(Op::ListOp(ListOp::Map)),
        "print" => Token::Op(Op::IO(IO::Print)),
        // if let guards is still experimental, but would be better
        // n if let Some(literal) = numberize(n) => Statement::Exp(Exp::Literal(literal)),
        // this way is a bit smelly since were parsing the number twice
        n if numberize(n).is_some() => {
            let literal = numberize(n).unwrap();
            Token::Exp(Exp::Literal(literal))
        }
        // if it doesnt match anything, we'll make  it a symbol/variable
        var_name => Token::Exp(Exp::Symbol(var_name.to_string())),
    })
}

fn parse_code_block(tokens: &mut Vec<&str>) -> Result<Vec<Token>, ParserError> {
    let mut block: Vec<Token> = Vec::new();
    while let Some(word) = tokens.last() {
        match *word {
            "}" => {
                tokens.pop();
                return Ok(block);
            }
            // this allows for recursive code blocks
            _ => block.push(tokenize_next(tokens)?),
        };
    }
    Err(ParserError::CodeBlockNotClosed)
}

fn parse_array(tokens: &mut Vec<&str>) -> Result<Vec<Exp>, ParserError> {
    let mut array: Vec<Exp> = Vec::new();
    while let Some(token) = tokens.last() {
        match *token {
            "]" => {
                tokens.pop();
                return Ok(array);
            }
            _ => match tokenize_next(tokens)? {
                Token::Exp(exp) => array.push(exp),
                Token::Op(_) => return Err(ParserError::OperationInArray),
            },
        };
    }
    Err(ParserError::ArrayNotClosed)
}

fn parse_string(tokens: &mut Vec<&str>) -> Result<String, ParserError> {
    let mut string = String::new();

    while let Some(word) = tokens.pop() {
        match word {
            "\"" => return Ok(string),
            // add space
            _ => {
                if !string.is_empty() {
                    string.push(' ');
                }
                string.push_str(word);
            }
        };
    }
    Err(ParserError::StringNotClosed)
}

fn numberize(n: &str) -> Option<Literal> {
    if n.contains('.') {
        match n.parse::<f64>() {
            Ok(val) => Some(Literal::Float(val)),
            Err(_) => None,
        }
    } else {
        match n.parse::<i128>() {
            Ok(val) => Some(Literal::Int(val)),
            Err(_) => None,
        }
    }
}
