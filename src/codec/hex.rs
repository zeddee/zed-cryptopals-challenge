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
            .flat_map(|c| vec![c & 0b00001111, (c & 0b11110000) >> 4])
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

