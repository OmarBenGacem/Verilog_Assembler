use std::fmt;
use std::println;
use crate::tokens::literal::Literal;
// #[allow(dead_code, unused_variables, unused_imports)]


pub struct Register {
    pub label : String,
    pub width : u32,
    pub value : Number,
}

pub fn reg_init(width: u32) -> Register {


    
}