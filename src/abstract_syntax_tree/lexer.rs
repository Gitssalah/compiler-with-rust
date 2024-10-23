use crate::abstract_syntax_tree::lexer::TokenKey::{Begin, End, Error, NewLine, Number, Plus, Print, Variable, VariableWithValue};
use crate::error_handler::error_controller::error_controller;
use crate::error_handler::error_controller::error_handler;

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
    Number(i64),
    VariableWithValue(String,i64),
    NewLine,
    Print,
    Error(String),
    Begin,
    End,
}

pub struct  Token {
    token_type: TokenKey,
    value: Option<String>, // the value in str then it should be compiled by rustc
}

pub fn tokenize(elem: String) -> TokenKey {
    if elem.parse::<i64>().is_ok() {
        Number(elem.parse::<i64>().unwrap())
    }
    else {
        match elem.as_str() {
            "+" => Plus,
            "\n" => NewLine,
            _ => Variable(elem)
        }
    }
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
            // tokenized_list.push(NewLine);
            line.remove(0);
            line.remove(0);
            tokenized_list.push(NewLine);
        }
        if line.contains("print"){
            line.drain(0..6);
            line.remove(line.len()-1);
            tokenized_list.push(Print);
        }
        if line.contains("=") {
            let mut buff: Vec<String> = line.split('=').map(|s| s.to_string()).collect();
            vars.push((buff[0].clone(), buff[1].parse().unwrap()));
            tokenized_list.push(VariableWithValue(buff[0].clone(), buff[1].parse().unwrap())); // adding variable name and value  need refactoring
            println!("hi {:?}", vars);

        }
        else if line.contains(("+")) {
            let mut buff: Vec<String>=line.split('+').map(|x| x.to_string()).collect();
            tokenized_list.push(tokenize(buff[0].clone()));
            for i in 1..buff.len() {
                tokenized_list.push(Plus);
                tokenized_list.push(tokenize(buff[i].clone()));
            }
        }
        else {
            let err = Error(line);
            error_handler(error_controller(&err));
            tokenized_list.push(err);
        }

    }
    tokenized_list.push(End);
    return tokenized_list
}