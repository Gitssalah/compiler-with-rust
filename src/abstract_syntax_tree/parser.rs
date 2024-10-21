use crate::abstract_syntax_tree::lexer::{Token, TokenKey};

//add the interpretation of the each token using consume
pub fn consume (token: TokenKey) -> Option<String>{

    match token {
        TokenKey::Plus=> Some(String::from("+")),
        TokenKey::Minus=> Some(String::from("-")),
        TokenKey::Asterisk=> Some(String::from("*")),
        TokenKey::Slash=> Some(String::from("/")),
        TokenKey::Equal=> Some(String::from("=")),
        TokenKey::SemiColumn=> Some(String::from(";")),
        TokenKey::NewLine=> Some(String::from("\n")),
        TokenKey::Number(value) => Some(value.to_string()),
        TokenKey::Variable(value) =>Some(value),
        TokenKey::VariableWithValue(first_value,second_value) =>Some(format!("let mut {} = {}", first_value, second_value)),
        _ => None
    }
}