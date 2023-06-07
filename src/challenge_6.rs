use std::collections::BinaryHeap;

use crate::{
    challenge_3::{sbx_recover_key, score_english},
    challenge_5::repeating_key_xor,
};

pub fn recover_rkx_key(input: &[u8]) -> Vec<u8> {
    let likely_keysizes = compute_likely_keysizes(input);
    println!("likely keysizes: {:?}", likely_keysizes);

    let mut best_key: Option<Vec<u8>> = None;
    let mut best_score = f32::NEG_INFINITY;

    for keysize in likely_keysizes {
        let mut key = Vec::<u8>::new();
        for block_idx in 0..keysize {
            let mut block = Vec::<u8>::new();
            let mut idx: usize = block_idx;
            while idx < input.len() {
                block.push(input[idx]);

                idx += keysize;
            }
            key.push(sbx_recover_key(&block));
        }
        let plaintext = repeating_key_xor(&input, &key);
        let score = score_english(&plaintext);

        if score > best_score {
            best_key = Some(key);
            best_score = score;
        }
    }

    best_key.unwrap()
}

#[derive(PartialEq, Debug)]
struct WeightedKeysize {
    keysize: usize,
    hamming_distance: f32,
}

impl Eq for WeightedKeysize {}

// doing some horrible things because f32 doesn't impl Ord or Eq apparently
impl Ord for WeightedKeysize {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .hamming_distance
            .partial_cmp(&self.hamming_distance)
            .unwrap()
            .then_with(|| self.keysize.cmp(&other.keysize))
    }
}

impl PartialOrd for WeightedKeysize {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn compute_likely_keysizes(input: &[u8]) -> Vec<usize> {
    let mut sizes: BinaryHeap<WeightedKeysize> = BinaryHeap::new();
    for keysize in 2..40 {
        if keysize * 4 >= input.len() {
            break;
        }
        let a = &input[0..keysize];
        let b = &input[keysize..keysize * 2];
        let c = &input[keysize * 2..keysize * 3];
        let d = &input[keysize * 3..keysize * 4];
        let ab = hamming_distance(a, b) / keysize as f32;
        let bc = hamming_distance(b, c) / keysize as f32;
        let cd = hamming_distance(c, d) / keysize as f32;

        let avg = ab + bc + cd / 3.0;

        sizes.push(WeightedKeysize {
            hamming_distance: avg,
            keysize,
        });
    }
    println!("{:?}", sizes);

    sizes
        .iter()
        .take(10)
        .map(|s| s.keysize)
        .collect::<Vec<usize>>()
}

fn hamming_distance(a: &[u8], b: &[u8]) -> f32 {
    assert_eq!(a.len(), b.len());

    let mut out: f32 = 0.0;
    for (i, x) in a.iter().enumerate() {
        let y = b[i];
        out += (x ^ y).count_ones() as f32;
    }

    out
}

#[cfg(test)]
mod tests {
    use std::fs;

    use crate::challenge_1::b64_to_bytes;

    use super::*;

    #[test]
    fn test_recover_rkx_key() {
        let raw_file = fs::read_to_string("data/6.txt").unwrap();
        let enc = b64_to_bytes(&raw_file);
        let key = recover_rkx_key(&enc);

        let dec = repeating_key_xor(&enc, &key);
        println!("{}", String::from_utf8_lossy(&dec));

        assert_eq!(key, [b'a', b'b', b'c']);
    }

    #[test]
    fn test_hamming_distance() {
        assert_eq!(hamming_distance(b"wokka wokka!!!", b"this is a test"), 37.0);
    }
}
