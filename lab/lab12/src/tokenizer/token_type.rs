use std::fmt;
// use std::ops::Add;

#[derive(Debug, Clone)]
pub enum Token {
    Exp(Exp),
    Op(Op),
}

#[derive(Debug, Clone)]
pub enum Exp {
    Literal(Literal),
    List(Vec<Exp>),
    CodeBlock(Vec<Token>),
    Symbol(String),
}

#[derive(Debug, Clone)]
pub enum Arithmetic {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
}

#[derive(Debug, Clone)]
pub enum Logical {
    And,
    Or,
    Not,
}

#[derive(Debug, Clone)]
pub enum Comparison {
    Equal,
    Greater,
    Less,
}

#[derive(Debug, Clone)]
pub enum StackOp {
    Pop,
    Swap,
    Dup,
}

#[derive(Debug, Clone)]
pub enum FlowOp {
    Exec,
    If,
    For,
}

#[derive(Debug, Clone)]
pub enum VarOp {
    AssignVar,
    AssignFun,
}

#[derive(Debug, Clone)]
pub enum ListOp {
    Length,
    Head,
    Map,
}

#[derive(Debug, Clone)]
pub enum Op {
    Arithmetic(Arithmetic),
    Logical(Logical),
    Comparison(Comparison),
    StackOp(StackOp),
    FlowOp(FlowOp),
    VarOp(VarOp),
    ListOp(ListOp),
    IO(IO),
}

#[derive(Debug, Clone)]
pub enum IO {
    Print,
}

#[derive(Debug, Clone)]
pub enum Literal {
    Int(i128),
    Float(f64),
    Bool(bool),
    String(String),
}

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Literal::Int(num) => write!(f, "{}", num),
            Literal::Float(num) => write!(f, "{:?}", num),
            Literal::Bool(state) => write!(f, "{}", if *state { "True" } else { "False" }),
            Literal::String(string) => write!(f, "\"{}\"", string),
        }
    }
}

impl fmt::Display for Arithmetic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Arithmetic::Add => write!(f, "+"),
            Arithmetic::Sub => write!(f, "-"),
            Arithmetic::Mul => write!(f, "*"),
            Arithmetic::Div => write!(f, "/"),
            Arithmetic::Mod => write!(f, "%"),
        }
    }
}

impl fmt::Display for Logical {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Logical::Or => write!(f, "or"),
            Logical::And => write!(f, "and"),
            Logical::Not => write!(f, "not"),
        }
    }
}

impl fmt::Display for Comparison {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Comparison::Equal => write!(f, "=="),
            Comparison::Greater => write!(f, ">"),
            Comparison::Less => write!(f, "<"),
        }
    }
}
impl fmt::Display for StackOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StackOp::Pop => write!(f, "pop"),
            StackOp::Swap => write!(f, "swap"),
            StackOp::Dup => write!(f, "dup"),
        }
    }
}

impl fmt::Display for FlowOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FlowOp::Exec => write!(f, "exec"),
            FlowOp::If => write!(f, "if"),
            FlowOp::For => write!(f, "for"),
        }
    }
}

impl fmt::Display for Op {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Op::Arithmetic(arithmetic) => write!(f, "{}", arithmetic),
            Op::Logical(logical) => write!(f, "{}", logical),
            Op::Comparison(comparison) => write!(f, "{}", comparison),
            Op::StackOp(stack_op) => write!(f, "{:?}", stack_op),
            Op::FlowOp(flow_op) => write!(f, "{:?}", flow_op),
            Op::VarOp(var_op) => write!(f, "{:?}", var_op),
            Op::ListOp(list_op) => write!(f, "{:?}", list_op),
            Op::IO(io) => write!(f, "{:?}", io),
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::Exp(exp) => write!(f, "{}", exp),
            Token::Op(op) => write!(f, "{}", op),
        }
    }
}

impl fmt::Display for Exp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Exp::Literal(literal) => write!(f, "{}", literal),
            Exp::CodeBlock(block) => {
                write!(f, "{{ ")?;
                for statement in block {
                    write!(f, "{} ", statement)?;
                }
                write!(f, "}}")
            }
            Exp::List(array) => {
                let content = array
                    .iter()
                    .map(|exp| exp.to_string())
                    .collect::<Vec<String>>()
                    .join(",");
                write!(f, "[{}]", content)
            }
            Exp::Symbol(symbol) => write!(f, "{}", symbol),
        }
    }
}
