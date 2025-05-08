use std::io::{self, Write};
use unicode_segmentation::UnicodeSegmentation;
use utils::getFileName;
mod utils;

fn main() {
    println!("Welcome to SpellChecker!\n");
    let file_name = getFileName();
}
