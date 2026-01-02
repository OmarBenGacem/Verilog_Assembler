use logos::Lexer;
use crate::lexer::Token;


#[derive(Debug)]
pub enum Keyword{
    Definition(Definition),
    Controlflow(Controlflow),
    Port(Port),
    Grammar(Grammar)
}


#[derive(Debug)]
pub enum Grammar {
    Semicolon { value : String },
    Colon { value : String },
    At { value : String },
    Openbracket {value : String},
    Closebracket {value : String}
}


#[derive(Debug)]
pub enum Definition {
    Assign {value: String},
    Module {value: String},
    Endmodule {value: String},
    Unknown { value: String }
}   



#[derive(Debug)]
pub enum Port {
    Input {value: String},
    Output {value : String},
    Reg   { value : String},
    Wire  {value : String}
}

#[derive(Debug)]
pub enum Controlflow {
    Case     {value: String},
    Endcase     {value: String},
    If        {value: String},
    Else      {value: String},
    Begin     {value: String},
    End     {value: String},
    Always  { value : String},
    Posedge { value : String },
    Negedge { value : String },
    Default     {value: String}, 
}

pub fn lex_keyword(lex: &mut Lexer<Token>) -> Keyword {
let slice = lex.slice().to_string();
    
    match lex.slice() {
        "assign"    => Keyword::Definition(Definition::Assign { value: slice }),
        "module"    => Keyword::Definition(Definition::Module { value: slice }),
        "endmodule" => Keyword::Definition(Definition::Endmodule { value: slice }),
        "case"      => Keyword::Controlflow(Controlflow::Case { value: slice }),
        "if"   => Keyword::Controlflow(Controlflow::If { value: slice }),
        "else"   => Keyword::Controlflow(Controlflow::Else { value: slice }),
        "begin"   => Keyword::Controlflow(Controlflow::Begin { value: slice }),
        "end"   => Keyword::Controlflow(Controlflow::End { value: slice }),
        "default"   => Keyword::Controlflow(Controlflow::Default { value: slice }),
        "posedge"  => Keyword::Controlflow( Controlflow::Posedge { value: slice } ),
        "negedge"  => Keyword::Controlflow( Controlflow::Negedge { value: slice } ),
        "always"  => Keyword::Controlflow( Controlflow::Always { value: slice } ),
        "input"  => Keyword::Port( Port::Input{value: slice} ),
        "output" => Keyword::Port( Port::Output{value: slice} ),
        "reg"  => Keyword::Port( Port::Reg{ value: slice } ),
        "wire" => Keyword::Port( Port::Wire { value : slice } ),
        ":"  => Keyword::Grammar( Grammar::Colon{ value : slice } ),
        ";"  => Keyword::Grammar( Grammar::Semicolon{ value : slice } ),
        "@"  => Keyword::Grammar( Grammar::At{ value : slice } ),
        "("  => Keyword::Grammar( Grammar::Openbracket{ value : slice } ),
        ")"  => Keyword::Grammar( Grammar::Closebracket { value : slice } ),

        _           => Keyword::Definition(Definition::Unknown { value: slice }),
    }
}



