use logos::Lexer;
use crate::lexer::Token;  // or wherever your Token enum lives

#[derive(Debug)]
pub enum Operator {
    Assign_block  { name: String, symbol: String },
    Equal_test { name: String, symbol: String },
    Assign_nblock { name: String, symbol: String },
    Plus         { name: String, symbol: String },
    Minus        { name: String, symbol: String },
    Multiply     { name: String, symbol: String },
    Xor          { name: String, symbol: String },
    Lshift       { name: String, symbol: String },
    Rshift       { name: String, symbol: String },
    Unknown      { name: String }
}   

pub fn lex_operator(lex: &mut Lexer<Token>) -> Operator {

    match lex.slice() {

        "=="  => Operator::Equal_test    { name: "equal_test".to_string(), symbol: "==".to_string() },
        "="   => Operator::Assign_block   { name: "assign".to_string(), symbol: "=".to_string() },
        "<="  => Operator::Assign_nblock  { name: "non-blocking asssign".to_string(), symbol: "<=" .to_string()},
        "+"   => Operator::Plus          { name: "plus".to_string(), symbol: "+".to_string() },
        "-"   => Operator::Minus         { name: "minus".to_string(), symbol: "-".to_string() },
        "*"   => Operator::Multiply      { name: "multiply".to_string(), symbol: "*".to_string() },
        "^"   => Operator::Xor           { name: "XOR".to_string(), symbol: "^".to_string() },
        "<<"  => Operator::Lshift        { name: "Left Shift".to_string(), symbol: "<<".to_string() },
        ">>"  => Operator::Rshift        { name: "Right Shift".to_string(), symbol: ">>".to_string() },
        other => Operator::Unknown { name: "ERROR".to_string() }

    }

}



 