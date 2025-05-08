use std::io::{self, Write};

pub fn getFileName() -> String {
    print!("Please input the file's name: ");
    io::stdout().flush().expect("Failed to print!");
    let mut file_name = String::new();
    io::stdin().read_line(&mut file_name).expect("Failed to read file name!");
    return file_name;
}