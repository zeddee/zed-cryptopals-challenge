/*
EncodingAdapter trait describes an interface for types
that implement an API for different representations of integer values.

E.g. Base64, Hexadecimal
*/
pub trait EncodingAdapter {
  fn value_to_char(&self, v: u8) -> Option<char>;
  fn char_to_value(&self, c: char) -> Option<u8>;
}
