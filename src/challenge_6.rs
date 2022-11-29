use std::collections::BTreeMap;

pub fn recover_rkx_key(input: &[u8]) -> Vec<u8> {
    let likely_keysizes = compute_likely_keysizes(input);
    println!("{:?}", likely_keysizes);

    vec![]
}

fn compute_likely_keysizes(input: &[u8]) -> Vec<usize> {
    // map of hamming distance -> keysize
    let mut sizes: BTreeMap<u32, usize> = BTreeMap::new();
    for keysize in 2..40 {
        if keysize * 2 >= input.len() {
            break;
        }
        let a = &input[0..keysize];
        let b = &input[keysize..keysize * 2];
        let distance = hamming_distance(a, b);

        // if the computed distance has already been seen, the smaller keysize
        // will stay (we'll see if this is a good idea when I get around to writing
        // the other functions).
        if sizes.contains_key(&distance) {
            continue;
        }
        sizes.insert(distance, keysize);
    }

    sizes.values().take(5).cloned().collect::<Vec<usize>>()
}

fn hamming_distance(a: &[u8], b: &[u8]) -> u32 {
    assert_eq!(a.len(), b.len());

    let mut out = 0;
    for (i, x) in a.iter().enumerate() {
        let y = b[i];
        out += (x ^ y).count_ones();
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

        assert_eq!(key, [b'a', b'b', b'c']);
    }

    #[test]
    fn test_hamming_distance() {
        assert_eq!(hamming_distance(b"wokka wokka!!!", b"this is a test"), 37);
    }
}
