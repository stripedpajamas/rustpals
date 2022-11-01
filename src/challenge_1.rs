const HEX: &[u8] = "0123456789abcdef".as_bytes();
const BASE64: &[u8] = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/".as_bytes();
const BASE64_PAD: char = '=';

pub fn bytes_to_hex(bytes: &[u8]) -> String {
    let mut out = String::new();
    for &x in bytes {
        out.push(HEX[(x / 16) as usize] as char);
        out.push(HEX[(x - ((x / 16) * 16)) as usize] as char);
    }

    out
}

pub fn bytes_to_b64(bytes: &[u8]) -> String {
    let mut out = String::new();
    for chunk in bytes.chunks(3) {
        out.push(BASE64[(chunk[0] >> 2) as usize] as char);
        if chunk.len() > 1 {
            out.push(BASE64[(((chunk[0] & 0x3) << 4) | (chunk[1] >> 4)) as usize] as char);
        } else {
            out.push(BASE64[((chunk[0] & 0x3) << 4) as usize] as char);
            out.push(BASE64_PAD);
            out.push(BASE64_PAD);
            return out;
        }
        if chunk.len() == 2 {
            out.push(BASE64[(((chunk[1] & 0xF) << 2) as usize)] as char);
            out.push(BASE64_PAD);
            return out;
        }
        out.push(BASE64[(((chunk[1] & 0xF) << 2) | (chunk[2] >> 6)) as usize] as char);
        out.push(BASE64[(chunk[2] & 0x3F) as usize] as char);
    }
    out
}

fn ascii_to_b64_index(c: u8) -> u8 {
    if c.is_ascii_uppercase() {
        c - 65 // A = 0
    } else if c.is_ascii_lowercase() {
        c - 71 // a = 26
    } else if c.is_ascii_digit() {
        c + 4 // 0 = 52
    } else {
        match c {
            b'+' => c + 19, // + = 62
            b'/' => c + 16, // / = 63
            _ => unreachable!(),
        }
    }
}

pub fn b64_to_bytes(b64_str: &str) -> Vec<u8> {
    assert!(b64_str.len() % 4 == 0);

    if b64_str.is_empty() {
        return Vec::new();
    }

    let mut out: Vec<u8> = Vec::new();

    for chunk in b64_str.as_bytes().chunks_exact(4) {
        out.push((ascii_to_b64_index(chunk[0]) << 2) | (ascii_to_b64_index(chunk[1]) >> 4));

        if chunk[2] == BASE64_PAD as u8 {
            return out;
        }

        out.push(((ascii_to_b64_index(chunk[1]) & 0xF) << 4) | (ascii_to_b64_index(chunk[2]) >> 2));

        if chunk[3] == BASE64_PAD as u8 {
            return out;
        }

        out.push(((ascii_to_b64_index(chunk[2]) & 0x3) << 6) | ascii_to_b64_index(chunk[3]));
    }

    out
}

pub fn hex_to_bytes(hex_str: &str) -> Vec<u8> {
    assert!(hex_str.len() % 2 == 0);

    if hex_str.is_empty() {
        return Vec::new();
    }

    let mut out_bytes: Vec<u8> = Vec::with_capacity(hex_str.len() / 2);
    for x in hex_str.as_bytes().chunks_exact(2) {
        assert!(x[0].is_ascii_hexdigit());
        assert!(x[1].is_ascii_hexdigit());

        let a = (if x[0].is_ascii_digit() {
            x[0] - 48
        } else {
            x[0].to_ascii_uppercase() - 55
        }) * 16;
        let b = if x[1].is_ascii_digit() {
            x[1] - 48
        } else {
            x[1].to_ascii_uppercase() - 55
        };

        out_bytes.push(a + b);
    }

    out_bytes
}

pub fn hex_to_b64(hex_str: &str) -> String {
    bytes_to_b64(&hex_to_bytes(hex_str))
}

pub fn b64_to_hex(b64_str: &str) -> String {
    bytes_to_hex(&b64_to_bytes(b64_str))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_bytes_to_b64() {
        assert_eq!(bytes_to_b64(&[b'M', b'a', b'n']), "TWFu");
        assert_eq!(bytes_to_b64(&[b'M', b'a']), "TWE=");
        assert_eq!(bytes_to_b64(&[b'M']), "TQ==");
        assert_eq!(bytes_to_b64(&[]), "");
    }

    #[test]
    fn test_b64_to_bytes() {
        assert_eq!(
            b64_to_bytes("bGlnaHQgd29y"),
            &[b'l', b'i', b'g', b'h', b't', b' ', b'w', b'o', b'r']
        );
        assert_eq!(
            b64_to_bytes("bGlnaHQgd28="),
            &[b'l', b'i', b'g', b'h', b't', b' ', b'w', b'o']
        );
        assert_eq!(
            b64_to_bytes("bGlnaHQgdw=="),
            &[b'l', b'i', b'g', b'h', b't', b' ', b'w']
        );
        assert_eq!(b64_to_bytes("TWFu"), &[b'M', b'a', b'n']);
        assert_eq!(b64_to_bytes("TWE="), &[b'M', b'a']);
        assert_eq!(b64_to_bytes("TQ=="), &[b'M']);
        assert_eq!(b64_to_bytes(""), &[]);
    }

    #[test]
    fn test_bytes_to_hex() {
        assert_eq!(bytes_to_hex(&[]), "");
        assert_eq!(bytes_to_hex(&[73]), "49");
        assert_eq!(bytes_to_hex(&[73, 39]), "4927");
        assert_eq!(bytes_to_hex(&[73, 39, 109]), "49276d");
    }

    #[test]
    fn test_hex_to_bytes() {
        assert_eq!(hex_to_bytes(""), &[]);
        assert_eq!(hex_to_bytes("49"), &[73]);
        assert_eq!(hex_to_bytes("4927"), &[73, 39]);
        assert_eq!(hex_to_bytes("49276d"), &[73, 39, 109]);
        assert_eq!(hex_to_bytes("49276D"), &[73, 39, 109]);
    }

    #[test]
    fn test_hex_to_b64() {
        assert_eq!(hex_to_b64(
            "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d"
        ), "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t");
    }

    #[test]
    fn test_b64_to_hex() {
        assert_eq!(b64_to_hex(
            "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t"
        ), "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d");
    }
}
