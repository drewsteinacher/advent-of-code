#[allow(unused_imports)]
#[macro_use]
extern crate maplit; // for hashmap macro for writing hashmap literals in tests

use std::fs;
use std::io;
use std::io::Read;
use std::collections::HashMap;

#[derive(Debug)]
enum CliError {
    IoError(io::Error)
}

impl From<io::Error> for CliError {
    fn from(error: io::Error) -> Self {
        CliError::IoError(error)
    }
}

fn main() -> Result<(), CliError> {
    
    let contents = import_file_string("input.txt")?;
    
    let checksum = compute_checksum(&contents)?;
    
    println!("Checksum: {}", checksum);
    
    Ok(())
}

fn import_file_string(filename: &str) -> Result<String, CliError> {
    
    let mut contents = String::new();
    fs::File::open(filename)?.read_to_string(&mut contents)?;
    
    Ok(contents)
}

fn letter_count(string: &str) -> Result<HashMap<char, u32>, CliError> {
    
    let mut letter_to_count = HashMap::new();
    
    for c in string.chars() {
        if !letter_to_count.contains_key(&c) {
            letter_to_count.insert(c,0);
        }
        letter_to_count.insert(c, letter_to_count[&c] + 1);
    }
    
    Ok(letter_to_count)
}

fn compute_checksum(string: &str) -> Result<u32, CliError> {
    
    let mut two_count: u32 = 0;
    let mut three_count = 0;
    
    for line in string.lines() {
        
        let letter_to_count = letter_count(line)?;
        
        if letter_to_count.values().filter(|&&x| x == 2).count() > 0 {
            two_count = two_count + 1;
        }
        if letter_to_count.values().filter(|&&x| x == 3).count() > 0 {
            three_count = three_count + 1;
        }
    }
    Ok(two_count * three_count)
}



#[cfg(test)]
mod tests {
    
    use super::{letter_count, compute_checksum};
    
    #[test]
    fn test_letter_count() {
        assert_eq!(
            hashmap!{'a' => 1, 'b' => 2, 'c' => 3},
            letter_count("abbccc").unwrap()
        );
        assert_eq!(
            hashmap!{'a' => 1, 'b' => 1, 'c' => 1, 'd' => 1, 'e' => 1, 'f' => 1},
            letter_count("abcdef").unwrap()
        );
        assert_eq!(
            hashmap!{'a' => 2, 'b' => 3, 'c' => 1},
            letter_count("bababc").unwrap()
        );
        assert_eq!(
            hashmap!{'a' => 1, 'b' => 2, 'c' => 1, 'd' => 1, 'e' => 1},
            letter_count("abbcde").unwrap()
        );
    }
    
    #[test]
    fn test_compute_checksum() {
        assert_eq!(
            12,
            compute_checksum("abcdef\nbababc\nabbcde\nabcccd\naabcdd\nabcdee\nababab").unwrap()
        );
    }
}