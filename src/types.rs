use std::error::Error;
use std::fmt;

// pub mod types {
// #[derive(Copy, Clone)]
#[derive(PartialEq)]
pub enum Expr {
    Assign(String,Box<Expr>),
    Lookup(String),
    Const(i32),
    Plus(Box<Expr>,Box<Expr>),
    Mult(Box<Expr>,Box<Expr>),
}
#[derive(Copy, Clone,Debug,PartialEq)]
pub enum Value {
    Num(i32),
    Bin(bool),
    // Str(String),
}
#[derive(Debug,PartialEq)]
pub enum BeepboopError {
    ParseError,
    SyntaxError,
}

impl Error for BeepboopError {
    // fn source(&self) -> Option<&(dyn Error + 'static)> {
    //     Some(
    // }
}

impl fmt::Display for BeepboopError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BeepboopError::ParseError => write!(f, "Parse error!"),
            BeepboopError::SyntaxError => write!(f, "Syntax error!"),
        }
    }
}
// }
