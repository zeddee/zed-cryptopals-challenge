use crate::codec::adapter::Codec;

pub fn encrypt<T: Codec>(codec: &T, data: &[u8], cipher: &[u8]) -> Vec<u8> {
    let _ = (codec, data, cipher);
    unimplemented!()
}

pub fn decrypt<T: Codec>(codec: &T, data: &[u8], cipher: &[u8]) -> Vec<u8> {
    let _ = (codec, data, cipher);
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::codec::hex::Hexadecimal;

    #[test]
    fn test_encrypt() {
        let input = "".as_bytes();
        let cipher = "".as_bytes();
        let expected =
            "Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal";

        assert_eq!(
            encrypt(&Hexadecimal {}, input, cipher)
                .iter()
                .map(|c| *c as char)
                .collect::<String>(),
            expected
        );
    }
}
