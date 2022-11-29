pub fn repeating_key_xor(input: &[u8], key: &[u8]) -> Vec<u8> {
    let mut out: Vec<u8> = Vec::with_capacity(input.len());

    for i in 0..input.len() {
        out.push(input[i] ^ key[i % key.len()]);
    }

    out
}

#[cfg(test)]
mod tests {
    use crate::challenge_1::hex_to_bytes;

    use super::*;

    #[test]
    fn test_rkx() {
        let input = "Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal";
        let key = "ICE";
        let expected_output = hex_to_bytes(
            "0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f"
        );

        assert_eq!(
            repeating_key_xor(input.as_bytes(), key.as_bytes()),
            expected_output
        );
    }
}
