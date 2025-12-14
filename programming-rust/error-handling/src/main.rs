fn main() {
    let error = JsonError {
        message: "Unexpected token".to_string(),
        line: 1,
        column: 1,
    };
    println!("Error: {:?}", error);
}

#[derive(Debug, Clone)]
pub struct JsonError {
    pub message: String,
    pub line: usize,
    pub column: usize,
}
