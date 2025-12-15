fn main() {
    let error = JsonError {
        message: "Unexpected token".to_string(),
        line: 1,
        column: 1,
    };
    println!("{}", error);
    eprintln!("{}", error);
}

use thiserror::Error;
#[derive(Error, Debug)]
#[error("JSON Error at line {line}, column {column}: {message}")]
pub struct JsonError {
    pub message: String,
    pub line: usize,
    pub column: usize,
}
