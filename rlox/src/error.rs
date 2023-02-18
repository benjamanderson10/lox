use std::fmt::Display;

pub struct ErrorHandler<T: Display> {
    errors: Vec<Error<T>>,
}

impl<T: Display> ErrorHandler<T> {
    
    pub fn new() -> ErrorHandler<T> {
        ErrorHandler { errors: Vec::new() }
    }
    
    pub fn push(&mut self, error: Error<T>) {
        self.errors.push(error);
    }
    
    pub fn report_errors(&self){
        for error in &self.errors {
            error.report();
        }
    }
}

pub struct Error<T: Display> {
    pub line: usize,
    pub column: usize,
    pub message: T,
}

pub enum ErrorType {
    Scanning,
    Parsing,
}

impl<T: Display> Error<T> {
    pub fn report(&self) {
        eprintln!("Error: {} @ {}:{}", self.message, self.line, self.column);
    }
}
