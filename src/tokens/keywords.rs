use logos::Lexer;
use crate::lexer::Token;

#[derive(Debug)]
pub enum Keyword {
    Assign {value: String},
    Module {value: String},
    Endmodule {value: String},
    Case     {value: String}, 
    Unknown { value: String }
}   

pub fn lex_keyword(lex: &mut Lexer<Token>) -> Keyword {
    // make the `match` the *last expression* of the function,
    // *without* a trailing semicolon — that way its result is returned:
    match lex.slice() {
        "assign"    => Keyword::Assign    { value: lex.slice().to_string() },
        "module"    => Keyword::Module    { value: lex.slice().to_string() },
        "endmodule" => Keyword::Endmodule { value: lex.slice().to_string() },
        "case"      => Keyword::Case      { value: lex.slice().to_string() },
        other => Keyword::Unknown   { value: lex.slice().to_string() }
        
    }
}



