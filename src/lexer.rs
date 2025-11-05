use logos::Logos;
use crate::tokens::literal::{lex_binary, lex_hex, lex_decimal, Literal};
use crate::tokens::keywords::{lex_keyword, Keyword};
use crate::tokens::operators::{lex_operator, Operator};

#[derive(Logos, Debug)]
pub enum Token {

    //(?i) means case insensitive

    /// collapse all number forms into one variant
    #[regex(r"(?i)\d+'b[01xz]+", lex_binary)]
    #[regex(r"(?i)\d+'h[0-9a-fxz]+", lex_hex)]
    #[regex(r"(?i)\d+'d[0-9xz]+", lex_decimal)]
    Literal(Literal),

    #[regex(r"(assign|module|endmodule)", lex_keyword)]
    Keyword(Keyword),

    #[regex(r"(=|<=|\+|-|\*|\^|<<|>>)", lex_operator)]
    Operator(Operator),


    #[regex(r"[ \t\n\r]+", logos::skip)]
    Skip,

    Error,
}

/// one‐pass lex → filter out whitespace/errors
pub fn lex(text: &str) -> Vec<Token> {
    Token::lexer(text)
         .filter_map(Result::ok)
         .filter(|t| !matches!(t, Token::Skip | Token::Error))
         .collect()
}
