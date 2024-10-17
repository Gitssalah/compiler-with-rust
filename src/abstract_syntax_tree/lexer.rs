use crate::abstract_syntax_tree::lexer::TokenKey::{Begin, End, Error, NewLine, Plus, Variable, VariableWithValue};

// Indicator of each key element
#[derive(Debug)]
pub enum TokenKey {
    Plus,
    Minus,
    Asterisk,
    Slash,
    Equal,
    SemiColumn,
    Variable(String),
    VariableWithValue(String,i64),
    NewLine,
    Error,
    Begin,
    End,
}

pub struct  Token {
    token_type: TokenKey,
    value: Option<String>, // the value in str then it should be compiled by rustc
}

pub fn lexer (content: String) -> Vec<TokenKey> {
    // todo: correct this garbage
    let mut file_content = content.replace(" ", "");
    let mut lines = file_content.split(';').map(|s| s.to_string());
    let mut vars: Vec<(String,i64)> = Vec::new();
    let mut tokenized_list: Vec<TokenKey> = vec![Begin];
    for mut line in lines {
        // collect the var and there value the issue is that I'm not using the tokens
        if line.contains("\n") {
            tokenized_list.push(NewLine);
            line.remove(0);
            line.remove(0);
        }
        if line.contains("=") {
            let mut buff: Vec<String> = line.split('=').map(|s| s.to_string()).collect();
            vars.push((buff[0].clone(), buff[1].parse().unwrap()));
            tokenized_list.push(VariableWithValue(buff[0].clone(), buff[1].parse().unwrap())); // adding variable name and value  need refactoring
            println!("hi {:?}", vars);

        }
        else if line.contains(("+")) {
            let mut buff: Vec<String>=line.split('+').map(|x| x.to_string()).collect();
            tokenized_list.push(Variable(buff[0].clone()));
            tokenized_list.push(Plus);
            tokenized_list.push(Variable(buff[1].clone()));
        }
        else {
            tokenized_list.push(Error);
        }

    }
    tokenized_list.push(End);
    return tokenized_list
}