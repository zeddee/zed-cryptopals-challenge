use super::adapter::Codec;

const UPPERCASEOFFSET: i8 = b'A' as i8; // b'A' is 65 in utf-8, but 0 in Base64. So the offset is b'A'-0.
const LOWERCASEOFFSET: i8 = b'a' as i8 - 26; // b'a' is 97 in utf-8, but represents 26 in Base64. So the offset is b'a'-26=71.
const DIGITOFFSET: i8 = b'0' as i8 - 52; // b'0' is 48 in utf-8, and represents 0 in Base64 (haha). So the offset is b'0'-52=-4
const PADDING: i8 = '=' as i8;

#[derive(Copy, Clone)]
pub struct Base64Adapter;

impl Codec for Base64Adapter {
    fn map_value_to_char(&self, v: u8) -> Option<u8> {
        let v = v as i8;
        let ascii_value = match v {
            0..=25 => v + UPPERCASEOFFSET,
            26..=51 => v + LOWERCASEOFFSET,
            52..=61 => v + DIGITOFFSET,
            62 => 43, // +
            63 => 47, // -

            _ => return None,
            /* We MUST not map any other characters outside the Base64 space.
             * per specification: https://www.rfc-editor.org/rfc/rfc4648#section-3.3
             *
             * > Non-alphabet characters could exist within base-encoded data,
             * > caused by data corruption or by design.  Non-alphabet characters may
             * > be exploited as a "covert channel", where non-protocol data can be
             * > sent for nefarious purposes.  Non-alphabet characters might also be
             * > sent in order to exploit implementation errors leading to, e.g.,
             * > buffer overflow attacks.
             */
        } as u8;

        Some(ascii_value)
    }

    fn map_char_to_value(&self, c: u8) -> Option<u8> {
        //https://base64.guru/learn/base64-characters
        let c = c as i8;
        let base64_index = match c {
            65..=90 => c - UPPERCASEOFFSET,
            97..=127 => c - LOWERCASEOFFSET,
            48..=57 => c - DIGITOFFSET,
            43 => 62, // '+'
            47 => 63, // '/'

            _ => return None, // also ignores PADDING
        } as u8;

        Some(base64_index)
    }

    /// Set expected length of byte chunks to 3.
    /// See [crate::codec::adapter::Codec::get_chunksize].
    fn get_chunksize(&self) -> usize {
        3
    }

    /// Bitwise operations to expand data in 3-byte chunks operating
    /// in an 8-bit space to 4-byte chunks operating in a 6-bit space.
    fn raw_encode(&self, chunk: &[u8]) -> Vec<u8> {
        let mut res = match chunk.len() {
            1 => vec![(&chunk[0] & 0b11111100) >> 2, (&chunk[0] & 0b00000011) << 4],
            2 => vec![
                (&chunk[0] & 0b11111100) >> 2,
                (&chunk[0] & 0b00000011) << 4 | (&chunk[1] & 0b11110000) >> 4,
                (&chunk[1] & 0b00001111) << 2,
            ],
            3 => vec![
                (&chunk[0] & 0b11111100) >> 2,
                (&chunk[0] & 0b00000011) << 4 | (&chunk[1] & 0b11110000) >> 4,
                (&chunk[1] & 0b00001111) << 2 | (&chunk[2] & 0b11000000) >> 6,
                &chunk[2] & 0b00111111,
            ],
            _ => unreachable!(),
        } // after performing bitwise operations, map each resulting byte from u8 to base64 characters
        .iter()
        .filter_map(|c| self.map_value_to_char(*c))
        .collect::<Vec<u8>>();

        while res.len() < 4 {
            res.push(PADDING as u8);
        } // Inelegant, but we MUST pad only after mapping values to base64 chars
          // because the padding characters must lie outside the mapped space.

        res
    }

    fn raw_decode(&self, chunk: &[u8]) -> Vec<u8> {
        match chunk.len() {
            2 => vec![
                (&chunk[0] & 0b00111111) << 2 | (&chunk[1] & 0b11110000) >> 4,
                (&chunk[1] & 0b00001111) << 4,
            ],
            3 => vec![
                (&chunk[0] & 0b00111111) << 2 | (&chunk[1] & 0b00110000) >> 4,
                (&chunk[1] & 0b00001111) << 4 | (&chunk[2] & 0b00111100) >> 2,
                (&chunk[2] & 0b00000011) << 6,
            ],
            4 => vec![
                (&chunk[0] & 0b00111111) << 2 | (&chunk[1] & 0b00110000) >> 4,
                (&chunk[1] & 0b00001111) << 4 | (&chunk[2] & 0b00111100) >> 2,
                (&chunk[2] & 0b00000011) << 6 | (&chunk[3] & 0b00111111),
            ],
            _ => unreachable!(),
        }
        .into_iter()
        .filter(|c| *c > 0) // strip empty chunks post-decocde (EOL chars)
        .collect()
    }

    /// Explicitly rewrite the [crate::codec::adapter::Codec::decode] method,
    /// because Base64 requires a different sequence of operations over
    /// the processed byte chunks.
    fn decode(&self, data: &[u8]) -> Vec<u8> {
        data.chunks(4)
            .map(|c| {
                // retain chunk size by stripping padding and remapping
                // within a map
                c.iter()
                    .filter(|c| **c != PADDING as u8)
                    .filter_map(|c| self.map_char_to_value(*c))
                    .collect::<Vec<u8>>()
            })
            .map(|c| self.raw_decode(&c))
            .flatten()
            .collect::<Vec<u8>>()
    }
}

#[cfg(test)]
mod tests {
    use super::Base64Adapter;
    use crate::codec::adapter::Codec;

    fn factory() -> Base64Adapter {
        Base64Adapter {}
    }

    #[test]
    fn test_encode_single_char() {
        let input_str = "a";
        let expected = "YQ==";

        let input_data = input_str.as_bytes();

        assert_eq!(factory().encode_to_string(input_data), expected);
    }

    #[test]
    fn test_encode_two_chars() {
        let input_str = "ab";
        let expected = "YWI=";

        let input_data = input_str.as_bytes();

        assert_eq!(factory().encode_to_string(input_data), expected);
    }

    #[test]
    fn test_encode_three_chars() {
        let input_str = "abc";
        let expected = "YWJj";

        let input_data = input_str.as_bytes();

        assert_eq!(factory().encode_to_string(input_data), expected);
    }

    #[test]
    fn test_encode_short_string() {
        let input_str = "Hello, world!";
        let expected = "SGVsbG8sIHdvcmxkIQ==";

        let input = input_str.as_bytes();

        assert_eq!(factory().encode_to_string(input), expected);
    }

    #[test]
    fn test_encode_longer_string() {
        let input_str = "And here be a bit longer text. Let's see how it goes!";
        let expected = "QW5kIGhlcmUgYmUgYSBiaXQgbG9uZ2VyIHRleHQuIExldCdzIHNlZSBob3cgaXQgZ29lcyE=";

        let input_data = input_str.as_bytes();

        assert_eq!(factory().encode_to_string(input_data), expected);
    }

    #[test]
    fn test_decode_single_char() {
        let input_str = "YQ==";
        let expected = "a";

        let input_data = input_str.as_bytes();

        assert_eq!(factory().decode_to_string(input_data), expected);
    }

    #[test]
    fn test_decode_two_chars() {
        let input_str = "YWI=";
        let expected = "ab";

        let input_data = input_str.as_bytes();

        assert_eq!(factory().decode_to_string(input_data), expected);
    }

    #[test]
    fn test_decode_three_chars() {
        let input_str = "YWJj";
        let expected = "abc";

        let input_data = input_str.as_bytes();

        assert_eq!(factory().decode_to_string(input_data), expected);
    }

    #[test]
    fn test_decode_short_string() {
        let input_str = "SGVsbG8sIHdvcmxkIQ==";
        let expected = "Hello, world!";

        let input = input_str.as_bytes();

        assert_eq!(factory().decode_to_string(input), expected);
    }

    #[test]
    fn test_decode_longer_string() {
        let input_str = "QW5kIGhlcmUgYmUgYSBiaXQgbG9uZ2VyIHRleHQuIExldCdzIHNlZSBob3cgaXQgZ29lcyE=";
        let expected = "And here be a bit longer text. Let's see how it goes!";

        let input_data = input_str.as_bytes();

        assert_eq!(factory().decode_to_string(input_data), expected);
    }
}
