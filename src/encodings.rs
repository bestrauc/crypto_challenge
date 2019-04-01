pub mod base64;
pub mod hex;

#[cfg(test)]
mod tests {
    use super::hex::*;
    use super::base64::*;

    #[test]
    fn test_hex_to_base64() {
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
