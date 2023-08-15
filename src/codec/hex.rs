use super::adapter::Codec;

pub struct Hexadecimal;
const UPPERCASEOFFSET: u8 = b'A'; // 65 in utf8, but 10 in hex (hex is case insensitive)
const LOWERCASEOFFSET: u8 = b'a'; // 97 in utf8, but 10 in hex
const DIGITOFFSET: u8 = b'0'; // 48

impl Codec for Hexadecimal {
    fn get_chunksize(&self) -> usize {
        2
    }
    fn map_value_to_char(&self, v: u8) -> Option<u8> {
        match v {
            0..=9 => Some(v + DIGITOFFSET),
            10..=35 => Some(v - 10 + LOWERCASEOFFSET), // Hex is case insensitive
            _ => None,
        }
    }
    fn map_char_to_value(&self, c: u8) -> Option<u8> {
        match c {
            b'0'..=b'9' => Some(c - DIGITOFFSET),
            b'a'..=b'z' => Some(c - LOWERCASEOFFSET + 10), // Hex is case insensitive
            b'A'..=b'Z' => Some(c - UPPERCASEOFFSET + 10),
            _ => None,
        }
    }

    fn raw_encode(&self, chunk: &[u8]) -> Vec<u8> {
        chunk
            .iter()
            .flat_map(|c| vec![(c & 0b11110000) >> 4, c & 0b00001111])
            .filter_map(|c| self.map_value_to_char(c))
            .collect::<Vec<u8>>()
    }

    /*
    Takes a string of hexadecimal values and returns a vector of 8-bit/4 byte values.

    hexadecimal strings are a series of 4 bit values
    represented in an 8-bit space.
    So when we decode hex to bytes, we want to remove the 4 bits of padding
    and concatenate each of the actual important 4 bits into an 8-bit space.
    */
    fn raw_decode(&self, data: &[u8]) -> Vec<u8> {
        let mut raw = data.iter()
            .filter_map(|c| self.map_char_to_value(*c));

        let mut res: Vec<u8> = Vec::new();
        while let (Some(h), Some(l)) = (raw.next(), raw.next()) {
            res.push((h & 0b00001111) << 4 | (l & 0b00001111))
        }
        res
    }
}


#[cfg(test)]
mod tests {
    use super::Hexadecimal;
    use crate::codec::adapter::Codec;

    fn factory() -> Hexadecimal {
        Hexadecimal {}
    }

    #[test]
    fn test_encode_single_char() {
        let input_str = "a";
        let expected = "61";

        let input_data = input_str.as_bytes();

        assert_eq!(factory().encode_to_string(input_data), expected);
    }

    #[test]
    fn test_encode_two_chars() {
        let input_str = "ab";
        let expected = "6162";

        let input_data = input_str.as_bytes();

        assert_eq!(factory().encode_to_string(input_data), expected);
    }

    #[test]
    fn test_encode_three_chars() {
        let input_str = "abc";
        let expected = "616263";

        let input_data = input_str.as_bytes();

        assert_eq!(factory().encode_to_string(input_data), expected);
    }

    #[test]
    fn tests_encode_short_string() {
        let input_str = "Hello, world!";
        let expected = "48656C6C6F2C20776F726C6421".to_lowercase();

        let input = input_str.as_bytes();

        assert_eq!(factory().encode_to_string(input), expected);
    }

    #[test]
    fn test_encode_longer_string() {
        let input_str = "And here be a bit longer text. Let's see how it goes!";
        let expected = "416E642068657265206265206120626974206C6F6E67657220746578742E204C657427732073656520686F7720697420676F657321".to_lowercase();

        let input_data = input_str.as_bytes();

        assert_eq!(factory().encode_to_string(input_data), expected);
    }
}
