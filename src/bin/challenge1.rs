use hex;
use base64;

fn main() {
    println!("Challenge 1 : {}", hex_2_b64("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d"));
}

fn hex_2_b64(input: &str) -> String {
   
    let decoded: Vec<u8> = hex::decode(input).unwrap();

    return base64::encode(decoded); 
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn challenge1() {
     
        let input = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
        let output = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";

        assert_eq!(hex_2_b64(input), output);

    }

}
