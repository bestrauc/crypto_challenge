use std::char;

/// Types for which this trait is implemented can be serialized as Hex strings.
trait HexPrintable {
    fn as_hex(&self) -> String;
}

impl HexPrintable for Vec<u8> {
    fn as_hex(&self) -> String {
        self.iter()
            .flat_map(|b| vec![b & 15, b >> 4])
            .map(|b| char::from_digit(b as u32, 16).unwrap())
            .collect::<String>()
    }
}


/// Decodes the bytes from a hex string.
trait FromHex {
    fn from_hex(&self) -> Vec<u8>;
}

impl FromHex for &str {
    fn from_hex(&self) -> Vec<u8> {
        // map the hex characters to numbers (e.g. 1 -> 1, A -> 10, F -> 15)
        let hex_bytes = self.chars()
            .map(|c| c.to_digit(16).unwrap() as u8)
            .collect::<Vec<u8>>();

        // each two hex characters represent one byte
        hex_bytes.chunks(2)
            .map(|chunk| (chunk[0] << 4) + chunk[1])
            .collect::<Vec<u8>>()
    }
}

/// Types for which this trait is implemented can be serialized as Base64 strings.
trait Base64Encoding {
    fn to_base64(&self) -> String;
}

fn base64_table(value: u8) -> char {
    match value {
        0..=25 => (value + 65) as char,
        26..=51 => ((value - 26) + 97) as char,
        52..=61 => ((value - 52) + 48) as char,
        62 => '+',
        63 => '/',
        _ => panic!("base64_table should only be called with 6-bit values")
    }
}


/// Basic base64 encoding implementation for byte vectors.
impl Base64Encoding for Vec<u8> {
    fn to_base64(&self) -> String {
        // base64 simply maps groups of 6 bits to a table of 64 characters
        // if the number of bytes isn't divisible by 3 (or 3*6 bits, in base64),
        // then the bytes are padded with zeroes to a multiple of 3 bytes.
        // Up to two bytes at the end can be zeroes, which are encoded as '='.
        self.chunks(3).flat_map(|chunk| {
            // buffer to hold the 6 bit values extracted from the 3-byte chunks
            let mut buffer = Vec::with_capacity(4);

            // extract the first 6 bits and the 2 bits of the first byte
            buffer.push((chunk[0] & 0b11111100) >> 2);
            buffer.push((chunk[0] & 0b00000011) << 4);

            // incorporate bytes 2 and 3, if they exist
            if chunk.len() > 1 {
                buffer[1] += (chunk[1] & 0b11110000) >> 4;
                buffer.push((chunk[1] & 0b00001111) << 2);
            }

            if chunk.len() > 2 {
                buffer[2] += (chunk[2] & 0b11000000) >> 6;
                buffer.push(chunk[2] & 0b00111111);
            }

            // initialize vec with padding characters, will be overwritten if not needed
            let mut encoding_char_vec = vec!['='; 4];
            for (i, &b) in buffer.iter().enumerate() {
                encoding_char_vec[i] = base64_table(b);
            }

            encoding_char_vec
        }).collect::<String>()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hex_and_back() {
        let hex_tests = [
            "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d",
            "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f",
            "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f",
        ];

        let base64_tests = [
            "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t",
            "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb28=",
            "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hybw==",
        ];

        for (&hex, &base64) in hex_tests.iter().zip(base64_tests.iter()) {
            let test_bytes = hex.from_hex();
            let test_base64 = test_bytes.to_base64();
            assert_eq!(test_base64, base64);
        }
    }
}
