use std::io::{self, Write};

fn main() {
    println!("Hello from Rust in Docker! 🦀");
    // Принудительный сброс буфера вывода
    io::stdout().flush().unwrap();
}