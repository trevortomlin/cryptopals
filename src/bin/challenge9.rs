fn main() {
    
    println!("Challenge 9 : {}", pkcs_padding(b"YELLOW SUBMARINE", 20)); 

}

fn pkcs_padding(input: &[u8], padding: i32) -> String  {

    let difference = (padding - input.len() as i32 % padding) as u8;

    let mut bytes = input.to_vec();

    while bytes.len() < padding as usize {
        bytes.push(difference);
    }

    String::from_utf8(bytes).unwrap()

}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn challenge9() {
     
        let input = b"YELLOW SUBMARINE";
        let output = String::from_utf8(b"YELLOW SUBMARINE\x04\x04\x04\x04".to_vec()).unwrap();

        assert_eq!(pkcs_padding(input, 20), output);

    }

}
