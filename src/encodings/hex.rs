use std::char;

/// Types for which this trait is implemented can be serialized as Hex strings.
pub trait HexPrintable {
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
pub trait FromHex {
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