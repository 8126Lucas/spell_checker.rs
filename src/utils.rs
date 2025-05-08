use std::io::{self, Write, Read};
use std::env;
use std::path::{Path, PathBuf};

pub fn getCurrentDirectory() -> PathBuf {
    env::current_dir().unwrap()
}

pub fn getFileName() -> String {
    print!("Please input the file's name: ");
    io::stdout().flush().expect("Failed to print!");
    let mut file_name = String::new();
    io::stdin().read_line(&mut file_name).expect("Failed to read file name!");
    file_name = file_name.trim().to_string();
    file_name
}

pub fn getFileContent(dir_path: PathBuf, file_name: String) -> String {
    let clean_file_name = file_name.trim();
    let file_path = dir_path.join(clean_file_name);
    println!("{}", file_path.display());
    let file_content = std::fs::read_to_string(&file_path)
        .expect("The program should have been able to read the file!");
    println!("The file \"{file_name}\" has been read!");
    println!("File's content: {file_content}");
    file_content
}