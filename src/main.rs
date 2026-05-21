use std::fs;
use std::io::{self, Write};

fn main() {
    read_from_file();
}

fn read_from_file() -> io::Result<()> {
    let content = fs::read_to_string("example_file.txt")?;
    println!("{}", content);
    Ok(())
}

fn create_write_file() -> io::Result<()> {
    let mut file = fs::File::create("example_file.txt")?;

    file.write_all(b"Hello write to file\n")?;
    println!("File created");
    Ok(())
}
