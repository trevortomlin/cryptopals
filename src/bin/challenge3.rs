use hex;
use std::collections::HashMap;
use std::collections::BinaryHeap;

fn main() {
    println!("Challenge 3 : {}", single_xor("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736")); 
    
}

fn single_xor(input: &str) -> String {

    let lut = HashMap::from([
        ('E', 12.02),
        ('T', 9.10),
        ('A', 8.12),
        ('O', 7.68),
        ('I', 7.31),
        ('N', 6.95),
        ('S', 6.28),
        ('R', 6.02),
        ('H', 5.92),
        ('D', 4.32),
        ('L', 3.98),
        ('U', 2.88),
        ('C', 2.71),
        ('M', 2.61),
        ('F', 2.30),
        ('Y', 2.11),
        ('W', 2.09),
        ('G', 2.03),
        ('P', 1.82),
        ('B', 1.49),
        ('V', 1.11),
        ('K', 0.69),
        ('X', 0.17),
        ('Q', 0.11),
        ('J', 0.10),
        ('Z', 0.07)
    ]);

    let decoded: Vec<u8> = hex::decode(input).unwrap();

    let mut bheap = BinaryHeap::new();

    for key in 0..=255 {
        
        let mut tmp: Vec<u8> = Vec::new();
        let mut freq_map = HashMap::new();

        for c in &decoded {


            let new_c = c ^ key;
            tmp.push(new_c);
            *freq_map.entry((new_c as char).to_ascii_uppercase()).or_insert(0.0) += 1.0;

        }
        
        freq_map.retain(|k, _| (*k).is_ascii_alphabetic());

        let sum: f32 = freq_map.values().sum();

        freq_map = freq_map.iter().map(|(k, v)| ((*k as char), (*v as f32) / sum * 100.0)).collect();
        
        let chi = chi_squared(&freq_map, &lut);
        
        if tmp.iter().all(|&x| x.is_ascii_alphanumeric() || x.is_ascii_whitespace() || x.is_ascii_punctuation()) {
            bheap.push((-chi as i32, tmp));
        }

    }

    let (_, v) = bheap.pop().unwrap();
    let v_str = String::from_utf8(v).unwrap();

    v_str    

}

fn chi_squared(freq_map: &HashMap<char, f32>, lut: &HashMap<char, f32>) -> f32 {

    let mut sum = 0.0;

    for (&k, &v) in freq_map {
        if lut.contains_key(&k) {
            sum += f32::powi(v - lut.get(&k).unwrap(), 2) / lut.get(&k).unwrap();            
        } 

    }

    sum

}
