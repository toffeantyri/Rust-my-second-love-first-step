use std::io::{self, Write};
use std::{fs, string};

fn main() {
    stringer();
}

fn stringer() {
    let text = "Programmin on Rust language - its fun";
    let substring = "Rust";

    match text.find(substring) {
        Some(v) => println!("Substring {} is exist, position : {} ", substring, v),
        None => println!("Substring is not exist"),
    }
}

#[allow(dead_code)]
fn input_and_divide() {
    let mut input = String::new();

    println!("Input numerator: ");
    io::stdin().read_line(&mut input).expect("Input error");

    let numerator = parse_f64(&input);

    input.clear();

    println!("Input denominator: ");

    io::stdin().read_line(&mut input).expect("Input error");
    let denominator = parse_f64(&input);

    let result = numerator.and_then(|num| denominator.and_then(|den| divide(num, den)));

    match result {
        Ok(res) => println!("Result {}", res),
        Err(e) => println!("{}", e),
    }
}

#[allow(dead_code)]
fn divide(x: f64, y: f64) -> Result<f64, String> {
    if y == 0f64 {
        Err(String::from("Denominator equals 0"))
    } else {
        Ok(x / y)
    }
}

fn parse_f64(input: &str) -> Result<f64, String> {
    return input
        .trim()
        .parse::<f64>()
        .map_err(|_| String::from("Parce error"));
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
