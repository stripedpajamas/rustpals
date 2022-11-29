pub fn recover_rkx_key(input: &[u8]) -> Vec<u8> {
    vec![]
}

pub fn hamming_distance(a: &[u8], b: &[u8]) -> u32 {
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

    // #[test]
    // fn test_recover_rkx_key() {
    //     let raw_file = fs::read_to_string("data/6.txt").unwrap();
    //     let enc = b64_to_bytes(&raw_file);
    //     let key = recover_rkx_key(&enc);

    //     assert_eq!(key, [b'a', b'b', b'c']);
    // }

    #[test]
    fn test_hamming_distance() {
        assert_eq!(hamming_distance(b"wokka wokka!!!", b"this is a test"), 37);
    }
}
