#[allow(unused_imports)]
#[macro_use]
extern crate maplit; // for hashmap macro for writing hashmap literals in tests

#[macro_use]
extern crate itertools;

extern crate strsim;

extern crate array_tool;

use std::fs;
use std::io;
use std::io::Read;
use std::collections::HashMap;
use strsim::hamming;
use itertools::Itertools;

#[allow(unused_imports)]
use array_tool::vec::Intersect;

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
    
    let (id_one, id_two) = get_similar_ids(&contents)?;
    println!("Similar IDs:");
    println!("{}", id_one);
    println!("{}", id_two);
    println!();
    
    let common_characters = get_common_character_string(id_one, id_two);
    println!("Common characters between the two IDs:");
    println!("{}", common_characters);
    
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

fn get_similar_ids(string: &str) -> Result<(String, String), CliError> {
    
    let lines = string.lines().collect_vec();
    
    for (a, b) in iproduct!(&lines, &lines) {
        if hamming(a, b).unwrap() == 1 {
            return Ok((a.to_string(), b.to_string()))
        }
    }
    
    Ok(("a".to_string(), "b".to_string()))
}

fn get_common_character_string(a: String, b: String) -> String {
    let mut s = String::new();
    let other_characters = b.chars().collect_vec();
    for (i, char) in a.chars().enumerate() {
        if char == other_characters[i] {
            s.push(char);
        }
    }
    s
}


#[cfg(test)]
mod tests {
    
    use super::{letter_count, compute_checksum, get_similar_ids, get_common_character_string};
    
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
    
    #[test]
    fn test_get_similar_ids() {
        assert_eq!(
            ("fghij".to_string(), "fguij".to_string()),
            get_similar_ids("abcde\nfghij\nklmno\npqrst\nfguij\naxcye\nwvxyz").unwrap()
        )
    }
    
    #[test]
    fn test_get_common_character_string() {
        assert_eq!(
            "fgij".to_string(),
            get_common_character_string("fghij".to_string(), "fguij".to_string())
        )
    }
}