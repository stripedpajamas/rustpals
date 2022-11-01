pub fn fixed_xor(a: &[u8], b: &[u8]) -> Vec<u8> {
    assert_eq!(a.len(), b.len());

    let mut out: Vec<u8> = Vec::with_capacity(a.len());
    for i in 0..a.len() {
        out.push(a[i] ^ b[i]);
    }

    out
}

#[cfg(test)]
mod tests {
    use crate::challenge_1::hex_to_bytes;

    use super::*;

    #[test]
    fn test_fixed_xor() {
        assert_eq!(
            fixed_xor(
                &hex_to_bytes("1c0111001f010100061a024b53535009181c"),
                &hex_to_bytes("686974207468652062756c6c277320657965")
            ),
            hex_to_bytes("746865206b696420646f6e277420706c6179")
        );
    }
}
