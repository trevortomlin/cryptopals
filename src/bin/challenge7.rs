use openssl::symm::{decrypt, Cipher};
use std::fs;

fn main() {
    
    let input = fs::read_to_string("input/challenge7.txt").expect("Should have been able to read the file.");

    println!("Challenge 7 : {}", aes_ecb(&input)); 

}

fn aes_ecb(input: &str) -> String {

    const KEY: &[u8; 16] = b"YELLOW SUBMARINE";

    let bytes = base64::decode(input.to_string().replace("\n", "")).unwrap();
    
    let cipher = Cipher::aes_128_ecb();

    let cipher_text = decrypt(cipher, KEY, None, &bytes);

    String::from_utf8(cipher_text.unwrap()).unwrap()

} 
