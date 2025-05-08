use std::io::{self, Write, Read};
use std::fs::File;
use unicode_segmentation::UnicodeSegmentation;
mod utils;

fn main() {
    println!("Welcome to SpellChecker!\n");
    let dir_path = utils::getCurrentDirectory();
    let file_name = utils::getFileName();
    let file_content = utils::getFileContent(dir_path, file_name);
}
