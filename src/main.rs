mod abstract_syntax_tree;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::env;
mod error_handler;

//need to read a file using our lang (file.lang)
//lib used
fn main() {
    let current_dir = env::current_dir();
    println!("Current directory: {:?}", current_dir.unwrap());
    let file_contents = fs::read_to_string("./src/file.lang");
    let content = file_contents.unwrap();
    println!("file content \n {:?}", content);
    let result = abstract_syntax_tree::lexer::lexer(content);
    println!("result {:?}", result);
    let mut rs_file = File::create("file.rs").expect("Failed to create file");
    for res in result {
        match res {
            abstract_syntax_tree::lexer::TokenKey::Error(value) => {
                println!("Error");
                break
            },
            _=> {}
        }
        if let Some(value) = abstract_syntax_tree::parser::consume(res) {
            // Write value to the file
            rs_file.write(value.as_bytes()).expect("Failed to write to file");
        } else {
            // Do nothing or handle the None case as needed
            continue; // Explicitly continue to the next iteration
        }
    }
}
