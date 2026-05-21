use std::fs;
use std::io::{self, Write};

fn main() {
    create_write_file();
}

fn create_write_file() -> io::Result<()> {
    let mut file = fs::File::create("example_file.txt")?;

    file.write_all(b"Hello write to file\n")?;
    println!("File created");
    Ok(())
}
