use std::fs;
use std::io;
use std::num;
use std::io::Read;
use std::collections::HashSet;

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

    let contents = import_file_string("input.txt")?;
    
    let resulting_frequency = resulting_frequency(&contents)?;
    println!("Part 1: Resulting frequency: {}", resulting_frequency);
    
    let first_repeat = first_repeated_frequency(&contents)?;
    println!("Part 2: First repeated frequency: {}", first_repeat);
    
    Ok(())
}

fn import_file_string(filename: &str) -> Result<String, CliError> {
    let mut contents = String::new();
    fs::File::open(filename)?.read_to_string(&mut contents)?;
    Ok(contents)
}

fn resulting_frequency(input: &str) -> Result<i32, CliError> {
    let mut sum = 0;
    for line in input.lines() {
        let change : i32 = line.trim().parse()?;
        sum += change;
    }
    Ok(sum)
}

fn first_repeated_frequency(input: &str) -> Result<i32, CliError> {
    
    let mut sum = 0;
    let mut observed_frequencies: HashSet<i32> = vec!(sum).into_iter().collect();
    
    loop {
        for line in input.lines() {
            
            let change : i32 = line.trim().parse()?;
            sum += change;
            
            let is_new = !observed_frequencies.insert(sum);
            if is_new {
                return Ok(sum);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{resulting_frequency, first_repeated_frequency};
    
    #[test]
    fn test_resulting_frequency() {
        assert_eq!(resulting_frequency("+1\n+1\n+1").unwrap(), 3);
        assert_eq!(resulting_frequency("+1\n+1\n-2").unwrap(), 0);
        assert_eq!(resulting_frequency("-1\n-2\n-3").unwrap(), -6);
    }
    
    #[test]
    fn test_first_repeated_frequency() {
        assert_eq!(first_repeated_frequency("+1\n-1").unwrap(), 0);
        assert_eq!(first_repeated_frequency("+3\n+3\n+4\n-2\n-4").unwrap(), 10);
        assert_eq!(first_repeated_frequency("-6\n+3\n+8\n+5\n-6").unwrap(), 5);
        assert_eq!(first_repeated_frequency("+7\n+7\n-2\n-7\n-4").unwrap(), 14);
    }
}