pub enum ErrorType {
    SyntaxError(String),
    ValueError(String),
    TypeError(String)
}

// todo add a handler that will receive error from the enum of token and determine the error type