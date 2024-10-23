use crate::abstract_syntax_tree::lexer::TokenKey;
pub enum ErrorType {
    SyntaxError(String),
    ValueError(String),
    TypeError(String),
    None,
}

// todo add a handler that will receive error from the enum of token and determine the error type
pub fn error_controller(err: TokenKey) -> ErrorType {
    match err {
        TokenKey::Error(value) =>  ErrorType::SyntaxError(value),
        _=>ErrorType::None
    }

}

pub fn error_handler(err: ErrorType){
    match err {
        ErrorType::SyntaxError(value) => println!("Syntax error: {:?}", value),
        ErrorType::TypeError(value) => println!("Type error: {:?}", value),
        ErrorType::ValueError(value) => println!("Value error: {:?}", value),
        ErrorType::None => println!("Undefined error")
    }
}