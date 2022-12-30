use hex;

fn main() {
    println!("Challenge 5 : {}", repeating_xor("Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal"));
}

fn repeating_xor(input: &str) -> String {

    let encoded = input.as_bytes(); 

    let key = b"ICE";

    let xor: Vec<u8> = encoded.iter()
                              .enumerate()
                              .map(|(i, &x)| x ^ key[i % key.len()])
                              .collect();

    return hex::encode(xor); 
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn challenge5() {
     
        let input = "Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal";
        let output = "0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f";

        assert_eq!(repeating_xor(input), output);

    }

}
