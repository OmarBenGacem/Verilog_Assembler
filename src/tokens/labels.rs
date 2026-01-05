use logos::Lexer;
use crate::lexer::Token;  // or wherever your Token enum lives


#[derive(Debug)]
pub enum Label{

    Text { name: String }//,
//     BitRange {
//     min: u16,
//     max: u16,
//     range: u16
//     }

}
#[derive(Debug)]
pub struct BitRange{
    lower: u16,
    upper: u16,
    width: u16,
    found_match: String
    }

pub fn lex_label(lex: &mut Lexer<Token>) -> Label {

    Label::Text { name: lex.slice().to_string() }

}

pub fn lex_bit_range( lex: &mut Lexer<Token> ) -> BitRange {

    let slice = lex.slice();
    let parts: Vec<&str> = slice
        .trim_matches(|c| c == '[' || c == ']')
        .split(':')
        .collect();

    let upper: u16 = parts[0].parse().expect("Upper bound is not a number");
    let lower: u16 = parts[1].parse().expect("Lower bound is not a number");

    BitRange{ found_match: lex.slice().to_string(), lower: lower, upper: upper, width: upper - lower + 1 }

} 