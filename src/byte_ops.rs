use crate::encodings::hex;

pub trait XOR {
    fn xor(&self, other: &Self) -> Self;
}

impl XOR for Vec<u8> {
    fn xor(&self, rhs: &Vec<u8>) -> Vec<u8> {
        assert_eq!(self.len(), rhs.len());
        self.iter()
            .zip(rhs.iter())
            .map(|(l, r)| l ^ r)
            .collect::<Vec<_>>()
    }
}

#[cfg(test)]
mod tests {
    use super::XOR;
    use crate::encodings::hex::{FromHex, HexPrintable};

    #[test]
    fn test_byte_vec_xor() {
        let hex_string1 = "1c0111001f010100061a024b53535009181c";
        let hex_string2 = "686974207468652062756c6c277320657965";
        let xor_hex_string = "746865206b696420646f6e277420706c6179";

        let string_bytes1 = hex_string1.from_hex();
        let string_bytes2 = hex_string2.from_hex();

        let xor_bytes = string_bytes1.xor(&string_bytes2);

        assert_eq!(xor_bytes.as_hex(), xor_hex_string);
    }
}
