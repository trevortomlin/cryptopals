use hex;

fn main() {
    println!("Challenge 2 : {}", fixed_xor("1c0111001f010100061a024b53535009181c", "686974207468652062756c6c277320657965"));
}

fn fixed_xor(input1: &str, input2: &str) -> String {
   
    let decoded: Vec<u8> = hex::decode(input1).unwrap();
    let decoded2: Vec<u8> = hex::decode(input2).unwrap();

    let xor: Vec<u8> = decoded.iter()
                     .zip(decoded2.iter())
                     .map(|(&x, &y)| x ^ y)
                     .collect();

    return hex::encode(xor); 
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn challenge2() {
     
        let input1 = "1c0111001f010100061a024b53535009181c";
        let input2 = "686974207468652062756c6c277320657965";
        let output = "746865206b696420646f6e277420706c6179";

        assert_eq!(fixed_xor(input1, input2), output);

    }

}
