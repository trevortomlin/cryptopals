use std::collections::HashMap;
use std::fs;
use base64;

fn main() {

    let input = fs::read_to_string("input/challenge6.txt").expect("Should have been able to read the file.");

    println!("Challenge 6 : {}", break_repeating_xor(&input)); 
    
}

fn break_repeating_xor(input: &str) -> String {

    let bytes = base64::decode(input.to_string().replace("\n", "")).unwrap();

    let mut min_dist = f32::MAX;
    let mut min_keysize = i32::MAX;
    

    for keysize in 2..=40 {    
        let num_chunks = bytes.len() / keysize;
        
        let mut dist = 0.0;

        for i in 0..num_chunks - 1 {

            let first = &bytes[keysize * i..keysize * i + keysize];
            let second = &bytes[keysize * i + keysize..keysize * i + 2 * keysize];
            dist += hamming_distance(&first, &second) as f32 / keysize as f32;
        
        }

        dist /= num_chunks as f32;

        if dist < min_dist {
            min_dist = dist;
            min_keysize = keysize as i32;
        }

    }

    let blocks = bytes.chunks(min_keysize as usize).collect::<Vec<_>>();
    let mut transpose = vec![vec![b' '; blocks.len()]; blocks[0].len()];

    for row in 0..blocks.len() {

        for col in 0..blocks[row].len() {

            transpose[col][row] = blocks[row][col];

        }

    }

    let mut result = String::new();

    for block in transpose {

        let key = single_xor(&block);
        result.push(key);

    }

    result

}

fn hamming_distance(str1: &[u8], str2: &[u8]) -> i32 {

    str1.iter()
        .zip(str2.iter())
        .fold(0, |acc, (x, y)| acc + (x ^ y).count_ones() as i32)

}

fn single_xor(input: &[u8]) -> char {

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
    let mut best_key = 0u8;

    let decoded: Vec<u8> = input.to_vec(); 

    for key in 0..=255 {
        
        let mut tmp: Vec<u8> = Vec::new();
        let mut freq_map = HashMap::new();
 
        for c in &decoded {
 
            let new_c = c ^ key;
            tmp.push(new_c);
            *freq_map.entry((new_c as char).to_ascii_uppercase()).or_insert(0.0) += 1.0;
 
        }
        
        let sum: f32 = freq_map.values().sum();
 
        freq_map = freq_map.iter().map(|(k, v)| (*k , *v / sum * 100.0)).collect();
        
        let chi = chi_squared(&freq_map, &lut);

         if chi < best_chi {
             best_chi = chi;
             best_key = key;
         }
 
    }

    best_key as char
}

fn chi_squared(freq_map: &HashMap<char, f32>, lut: &HashMap<char, f32>) -> f32 {

    let mut sum = 0.0;

    for (&k, &v) in freq_map {
        
        if lut.contains_key(&k) {
            sum += f32::powi(v - lut.get(&k).unwrap(), 2) / lut.get(&k).unwrap();            
        }
        else {
            sum += 10.0;
        }
    }

    sum

}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn challenge6() {
     
        let input = b"this is a test";
        let input2 = b"wokka wokka!!!";
        let output = 37;

        assert_eq!(hamming_distance(input, input2), output);

    }

}
