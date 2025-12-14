use logos::Lexer;
use crate::lexer::Token;  // or wherever your Token enum lives

/// All the different forms of numeric literal in Verilog
#[derive(Debug)]
pub enum Literal {
    Binary { width: usize, value: String },
    Hex    { width: usize, value: String },
    Decimal{ width: usize, value: String },
}

/// callbacks for logos — each takes the matched slice and returns a `Literal`
pub fn lex_binary(lex: &mut Lexer<Token>) -> Literal {
    let slice = lex.slice();           // e.g. "8'b1010"
    let mut parts = slice.split('\''); // ["8", "b1010"]
    let width = parts.next().unwrap().parse().unwrap();
    let rest  = parts.next().unwrap(); // "b1010"
    Literal::Binary {
        width,
        value: rest[1..].to_string(), // drop the 'b'
    }
}

pub fn lex_hex(lex: &mut Lexer<Token>) -> Literal {
    let slice = lex.slice();           // e.g. "16'hF0xz"
    let mut parts = slice.split('\'');
    let width = parts.next().unwrap().parse().unwrap();
    let rest  = parts.next().unwrap(); // "hF0xz"
    Literal::Hex {
        width,
        value: rest[1..].to_string(), // drop the 'h'
    }
}

pub fn lex_decimal(lex: &mut Lexer<Token>) -> Literal {
    let slice = lex.slice();           // e.g. "5'd17"
    let mut parts = slice.split('\'');
    let width = parts.next().unwrap().parse().unwrap();
    let rest  = parts.next().unwrap(); // "d17"
    Literal::Decimal {
        width,
        value: rest[1..].to_string(), // drop the 'd'
    }
}
 