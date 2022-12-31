use std::fs;
use std::collections::HashSet;

fn main() {
    
    let input = fs::read_to_string("input/challenge8.txt").expect("Should have been able to read the file.");

    println!("Challenge 8 : {}", detect_aes_ecb(&input)); 

}

fn detect_aes_ecb(input: &str) -> String {

    let mut max_line = String::new();
    let mut max_identical = 0;

    for line in input.lines() {


        let bytes = base64::decode(line.to_string()).unwrap();

        let unique_blocks = bytes.chunks(16).collect::<HashSet<_>>().iter().count();

        let identical_blocks = bytes.len() - unique_blocks;

        if identical_blocks > max_identical {
            max_identical = identical_blocks;
            max_line = line.to_string();
        }

    }

    max_line

}
