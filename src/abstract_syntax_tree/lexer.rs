
// Indicator of each key element
pub enum TokenKey {
    Plus,
    Minus,
    Asterisk,
    Slash,
    Equal,
    SemiColumn,
    Number(i64),
}

pub struct  Token {
    token_type: TokenKey,
    value: Option<std::str>, // the value in str then it should be compiled by rustc
}

pub fn lexer (file_content: &str) -> &'static str {
    let mut c: &str = "";
    let mut my_vars: Vec<(str, i64)> = vec![];
    let mut ele: &str = "";
    let mut value: i64 = 0;
    let mut value_as_string: &str = "";
    let mut i: i64 = 0;
    // todo: correct this garbage
    while c != ";" {
        c = file_content[i];
        if c.is_alphabetic() { //first we assume that a variable can only contain letters
            ele += c;
        }
        else if c.is_degit(10) {
            value_as_string += c
        }
        else {

        }
    }
    return ""
}