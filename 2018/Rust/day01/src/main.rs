use std::fs;
use std::io;
use std::num;
use std::io::Read;

#[derive(Debug)]
enum CliError {
    IoError(io::Error),
    ParseError(num::ParseIntError)
}

impl From<io::Error> for CliError {
    fn from(error: io::Error) -> Self {
        CliError::IoError(error)
    }
}

impl From<num::ParseIntError> for CliError {
    fn from(error: num::ParseIntError) -> Self{
        CliError::ParseError(error)
    }
}

fn main() -> Result<(), CliError> {
    
    let path = "input.txt";
    let mut contents = String::new();
    fs::File::open(path)?.read_to_string(&mut contents)?;
    
    let mut sum = 0;
    for line in contents.lines() {
        let change : i32 = line.trim().parse()?;
        sum += change;
    }
    println!("Part 1: {}", sum);
    
    Ok(())
}