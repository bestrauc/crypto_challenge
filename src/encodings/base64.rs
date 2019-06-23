/// Types for which this trait is implemented can be serialized as Base64 strings.
pub trait Base64Encoding {
    fn to_base64(&self) -> String;
}

fn base64_table(value: u8) -> char {
    match value {
        0..=25 => (value + 65) as char,
        26..=51 => ((value - 26) + 97) as char,
        52..=61 => ((value - 52) + 48) as char,
        62 => '+',
        63 => '/',
        _ => panic!("base64_table should only be called with 6-bit values"),
    }
}

/// Basic base64 encoding implementation for byte vectors.
impl Base64Encoding for Vec<u8> {
    fn to_base64(&self) -> String {
        // base64 simply maps groups of 6 bits to a table of 64 characters
        // if the number of bytes isn't divisible by 3 (or 3*6 bits, in base64),
        // then the bytes are padded with zeroes to a multiple of 3 bytes.
        // Up to two bytes at the end can be zeroes, which are encoded as '='.
        self.chunks(3)
            .flat_map(|chunk| {
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
            })
            .collect::<String>()
    }
}
