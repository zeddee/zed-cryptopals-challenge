use crate::crack::xor_decrypt;

pub fn find_encrypted_string(filename: &str) -> String {
    /* One-off function to:
    1. Read a file containing one hex per line.
    1. Decrypt each line with xor_decrypt::brute.
    1. Score the result with xor_decrypt::decrypt_score.
    1. Return the decrypted string that scores the highest.
     */
    let mut res: (String, usize) = (String::from(""), 0);
    let crypt_list = crate::utils::read_file(filename);

    for crypt_line in crypt_list {
        let line_decrypt = xor_decrypt::brute(&crypt_line.as_str());
        let score = xor_decrypt::decrypt_score(line_decrypt.clone());
        let line_decrypt_string =
            line_decrypt.iter().map(|c| *c as char).collect::<String>();
        println!("decrypted: {}", line_decrypt_string);
        if score > res.1 {
            res = (line_decrypt_string, score);
        }
    }
    res.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_encrypted_string() {
        let res = find_encrypted_string("_data/set1/challenge4.txt");
        let expected = "Now that the party is jumping\n";
        assert_eq!(res, expected);
    }
}
