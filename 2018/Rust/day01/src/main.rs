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

    let contents = import_file_string("input.txt")?;
    
    let resulting_frequency = resulting_frequency(&contents)?;
    println!("Part 1: Resulting frequency: {}", resulting_frequency);
    
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

#[cfg(test)]
mod tests {
    use super::resulting_frequency;
    
    #[test]
    fn test_resulting_frequency() {
        assert_eq!(resulting_frequency("+1\n+1\n+1").unwrap(), 3);
        assert_eq!(resulting_frequency("+1\n+1\n-2").unwrap(), 0);
        assert_eq!(resulting_frequency("-1\n-2\n-3").unwrap(), -6);
    }
}

}