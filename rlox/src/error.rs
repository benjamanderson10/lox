use std::fmt::Display;

pub struct Error<T: Display> {
    pub line: usize,
    pub column: usize,
    pub message: T,
}

impl<T: Display> Error<T> {
    pub fn report(&self) {
        eprintln!("Error: {} @ {}:{}", self.message, self.line, self.column);
    }
}
