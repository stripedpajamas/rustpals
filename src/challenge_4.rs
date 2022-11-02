use crate::challenge_3::{sbx_recover_key, score_english, single_byte_xor};

/// Returns the index in the given list to the line that was encrypted with SBX
pub fn detect_sbx(enc_list: &Vec<Vec<u8>>) -> usize {
    let mut best_score_idx: (f32, usize) = (f32::MIN, 0);
    for (idx, s) in enc_list.iter().enumerate() {
        let key = sbx_recover_key(&s);
        let plaintext = single_byte_xor(&s, key);
        let score = score_english(&plaintext);

        if score > best_score_idx.0 {
            best_score_idx = (score, idx);
        }
    }
    best_score_idx.1
}

#[cfg(test)]
mod tests {
    use std::fs;

    use crate::{challenge_3::{single_byte_xor, sbx_recover_key}, challenge_1::hex_to_bytes};

    use super::*;

    #[test]
    fn test_detect_sbx() {
        let raw_file = fs::read_to_string("data/4.txt").unwrap();
        let enc_list: Vec<Vec<u8>> = raw_file.lines()
            .map(|line| { hex_to_bytes(line) })
            .collect();

        let res = detect_sbx(&enc_list);

        let key = sbx_recover_key(&enc_list[res]);
        let plaintext = single_byte_xor(&enc_list[res], key);
        assert_eq!(String::from_utf8_lossy(&plaintext), "Now that the party is jumping\n");
    }
}