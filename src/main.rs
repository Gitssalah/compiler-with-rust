mod abstract_syntax_tree;
use std::fs;
use std::env;
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
    for res in result {
        match abstract_syntax_tree::parser::consume(res) {
            Some(value) =>println!("tokens consumed {:?}", value),
            None =>print!("")
        }

    }
}
