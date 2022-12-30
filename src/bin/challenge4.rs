use hex;
use std::collections::HashMap;
use std::fs;

fn main() {

    let input = fs::read_to_string("challenge4.txt").expect("Should have been able to read the file.");

    println!("Challenge 4 : {}", single_xor(&input)); 
    
}

fn single_xor(input: &str) -> String {

    let lut = HashMap::from([
        (' ', 18.29),
        ('E', 10.27),
        ('T', 7.51),
        ('A', 6.53),
        ('O', 6.16),
        ('I', 6.67),
        ('N', 5.71),
        ('S', 5.32),
        ('R', 4.99),
        ('H', 4.98),
        ('D', 3.28),
        ('L', 3.31),
        ('U', 2.28),
        ('C', 2.23),
        ('M', 2.03),
        ('F', 1.98),
        ('Y', 1.42),
        ('W', 1.70),
        ('G', 1.62),
        ('P', 1.50),
        ('B', 1.26),
        ('V', 0.80),
        ('K', 0.56),
        ('X', 0.14),
        ('Q', 0.08),
        ('J', 0.10),
        ('Z', 0.05)
    ]);


    let mut best_chi = f32::MAX;
    let mut best_vec = Vec::new();

    for line in input.lines() {
    
        let decoded: Vec<u8> = hex::decode(line).unwrap();

        for key in 0..=255 {
            
            let mut tmp: Vec<u8> = Vec::new();
            let mut freq_map = HashMap::new();
    
            for c in &decoded {
    
    
                let new_c = c ^ key;
                tmp.push(new_c);
                *freq_map.entry((new_c as char).to_ascii_uppercase()).or_insert(0.0) += 1.0;
    
            }
            
            freq_map.retain(|k, _| (*k).is_ascii_alphabetic() || (*k).is_ascii_whitespace());
    
            let sum: f32 = freq_map.values().sum();
    
            freq_map = freq_map.iter().map(|(k, v)| ((*k as char), (*v as f32) / sum * 100.0)).collect();
            
            let chi = chi_squared(&freq_map, &lut);
            
            if tmp.iter().all(|&x| x.is_ascii_alphanumeric() || x.is_ascii_whitespace() || x.is_ascii_punctuation()) {
                if chi < best_chi {
                    best_chi = chi;
                    best_vec = tmp;
                }
            }
    
        }
    
    }

    best_vec.iter().cloned().map(|c| c as char).collect::<String>()

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
