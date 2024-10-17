
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
    value: Option<String>, // the value in str then it should be compiled by rustc
}

pub fn lexer (content: String) -> &'static str {
    // todo: correct this garbage
    let mut file_content = content.replace(" ", "");
    let mut lines = file_content.split(';').map(|s| s.to_string());
    let mut vars: Vec<(String,i64)> = Vec::new();
    for line in lines {
        // collect the var and there value the issue is that I'm not using the tokens
        if line.contains("=") {
            let mut buff: Vec<String> = line.split('=').map(|s| s.to_string()).collect();
            vars.push((buff[0].clone(), buff[1].parse().unwrap()));
            println!("hi {:?}", vars);

        }
        else if line.contains(("-")) {
            //same issue no use f token
        }
    }
    return "0"
}