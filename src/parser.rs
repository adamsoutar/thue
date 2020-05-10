use crate::rules::*;
use crate::char_stream;
use crate::char_stream::CharStream;

pub struct ThueProgram {
    pub rules: Vec<Rule>,
    pub input: String
}

pub fn parse (s: String) -> ThueProgram {
    let mut tP = ThueProgram {
        rules: vec![],
        input: String::from("")
    };

    
}
