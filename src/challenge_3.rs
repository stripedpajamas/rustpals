const LTR_FREQS: [f32; 29] = [
    0.08167, // a
    0.01492, // b
    0.02782, // c
    0.04253, // d
    0.12702, // e
    0.02228, // f
    0.02015, // g
    0.06094, // h
    0.06966, // i
    0.00153, // j
    0.00772, // k
    0.04025, // l
    0.02406, // m
    0.06749, // n
    0.07507, // o
    0.01929, // p
    0.00095, // q
    0.05987, // r
    0.06327, // s
    0.09056, // t
    0.02758, // u
    0.00978, // v
    0.02360, // w
    0.00150, // x
    0.01974, // y
    0.00074, // z
    0.12900, // <space>
    0.08000, // <digits/punc>
    0.00000, // non-ascii
];

pub fn single_byte_xor(a: &[u8], key: u8) -> Vec<u8> {
    let mut out: Vec<u8> = Vec::with_capacity(a.len());
    for b in a {
        out.push(b ^ key);
    }
    out
}

pub fn sbx_recover_key(ciphertext: &[u8]) -> u8 {
    let mut best_score_key: (f32, u8) = (f32::MIN, 0);
    for key in 0..255 {
        let plaintext = single_byte_xor(ciphertext, key);
        let eng_score = score_english(&plaintext);
        if eng_score > best_score_key.0 {
            best_score_key = (eng_score, key);
        }
    }
    best_score_key.1
}

pub fn score_english(text: &[u8]) -> f32 {
    let text_len = text.len() as f32;
    let mut sample_ltr_counts: [f32; LTR_FREQS.len()] = [0.0; LTR_FREQS.len()];
    for &x in text {
        sample_ltr_counts[ascii_to_freq_idx(x)] += 1.0;
    }
    let mut score: f32 = 0.0;
    for i in 0..LTR_FREQS.len() {
        let error = LTR_FREQS[i] - (sample_ltr_counts[i] / text_len);
        score -= error * error;
    }

    score
}

fn ascii_to_freq_idx(x: u8) -> usize {
    if x.is_ascii_alphabetic() {
        (x.to_ascii_uppercase() - 65) as usize
    } else if x == b' ' {
        26
    } else if x.is_ascii_digit() || x.is_ascii_punctuation() {
        27
    } else {
        28
    }
}

#[cfg(test)]
mod tests {
    use crate::challenge_1::hex_to_bytes;

    use super::*;

    #[test]
    fn test_single_byte_xor() {
        assert_eq!(single_byte_xor(&[0x5, 0x3, 0x5], 0x6), &[0x3, 0x5, 0x3]);
    }

    #[test]
    fn test_sbx_recover_key() {
        let ciphertext =
            hex_to_bytes("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736");
        let key = sbx_recover_key(&ciphertext);
        let plaintext = single_byte_xor(&ciphertext, key);

        assert_eq!(
            String::from_utf8_lossy(&plaintext),
            "Cooking MC's like a pound of bacon"
        );
    }
}
