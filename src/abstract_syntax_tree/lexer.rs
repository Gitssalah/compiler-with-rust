
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
    // todo: correct this garbage
    file_content = file_content.replace(" ", "");
    let mut lines = file_content.split(';').map(|s| s.to_string());
    let vars: [(str,i64)] = [];
    for line in lines {
        // collect the var and there valur the issue is that I'm not using the tokens
        if line.contains("=") {
            let buff = line.split('=').map(|s| s.to_string());
            var.push((buff[0]), buff[1].parse().unwrap())
        }
        else if line.contains(("-")) {
            //same issue no use f tokens

        }
    }
    return ""
}