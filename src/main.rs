use std::fs;
use std::io::{self, Write};

fn main() {
    remove_file();
}

fn remove_file() -> io::Result<()> {
    let path = "example_file.txt";
    fs::remove_file(path)?;
    Ok(())
}

fn check_file_exist() {
    let path = "example_file.txt";

    if fs::metadata(path).is_ok() {
        println!("File {} is exist ", path);
    } else {
        println!("File {} is not exist", path);
    }
}

#[allow(dead_code)]
fn read_from_file() -> io::Result<()> {
    let content = fs::read_to_string("example_file.txt")?;
    println!("{}", content);
    Ok(())
}

#[allow(dead_code)]
fn create_write_file() -> io::Result<()> {
    let mut file = fs::File::create("example_file.txt")?;

    file.write_all(b"Hello write to file\n")?;
    println!("File created");
    Ok(())
}
