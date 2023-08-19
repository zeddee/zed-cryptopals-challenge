

/// # Codec adapter
/// Implement Codec to provide an adapter that
/// implements the ability to encode and decode a given encoding format.
/// This module provides implementations for [Base64][crate::codec::b64::Base64Adapter]
/// and [Hexadecimal][crate::codec::hex::Hexadecimal] encodings.
/// 
/// Codecs should allow you to:
/// - Transform a byte slice of UTF-8 code points to code points for the target encoding format.
/// - Transform a byte slice of code points from the target encoding format to UTF-8 code points.
pub trait Codec {
    /// Provide a mapping from code points from the target encoding format
    /// to UTF-8 code points.
    fn map_value_to_char(&self, v: u8) -> Option<u8>;

    /// Provide a mapping from UTF-8 code points to code points for the target encoding format.
    fn map_char_to_value(&self, c: u8) -> Option<u8>;

    /// Provide low-level transformation of content
    /// from one encoding format to another
    /// by manipulating the bytes that represent that content,
    /// and then applying the mapping provided by [Codec::map_value_to_char].
    fn raw_encode(&self, v: &[u8]) -> Vec<u8>;

    /// Provide low-level transformation of content
    /// from one encoding format to another
    /// by manipulating the bytes that represent that content,
    /// and then applying the mapping provided by [Codec::map_value_to_char].
    fn raw_decode(&self, v: &[u8]) -> Vec<u8>;

    /// Helper that just stores and returns the number of bytes this Codec instance
    /// should expect to operate on for a given encoding/decoding implementation.
    ///
    /// Only called internally by [Codec::encode]
    /// to provide chunks of this size for [Codec::raw_encode] to operate over.
    /// 
    /// By default, returns `4` for the 4 byte chunks we expect to contain UTF8 characters.
    /// Change this when working with encodings that expect a different chunk size.
    /// For example:
    /// - [crate::codec::b64::Base64Adapter] changes this to 3 because Base64 encoding expects 3 byte chunks
    /// - [crate::codec::hex::Hexadecimal] changes this to 2 because Hexadecimal encoding expects 2 byte chunks
    /// 
    /// Used in lieu of being able to declare `const` values in a trait.
    fn get_chunksize(&self) -> usize { 4 }

    /// Encode a byte slice using [Codec::raw_encode].
    fn encode(&self, data: &[u8]) -> Vec<u8> {
        data.chunks(self.get_chunksize())
            .flat_map(|c| self.raw_encode(c))
            .collect::<Vec<u8>>()
    }

    /// Convenience function that wraps [Codec::encode]
    /// to encode a byte slice as a String in the target encoding format.
    fn encode_to_string(&self, data: &[u8]) -> String {
        self.encode(data)
            .iter()
            .map(|v| (*v as char).to_string())
            .collect::<String>()
    }

    /// Encode a byte slice using [Codec::raw_decode].
    fn decode(&self, data: &[u8]) -> Vec<u8> {
        data.chunks(4)
            .flat_map(|c| self.raw_decode(c))
            .collect::<Vec<u8>>()
    }

    /// Convenience function that wraps [Codec::decode]
    /// to encode a byte slice as a String in the target encoding format.
    fn decode_to_string(&self, data: &[u8]) -> String {
        self.decode(data)
            .iter()
            .map(|v| (*v as char).to_string())
            .collect::<String>()
    }
}
