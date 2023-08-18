use crate::crack::xor_decrypt::decrypt_score;


pub fn find_encrypted_string(filename: &str) -> String {
    let mut res: (String, usize) = (String::from(""), 0);
    match crate::utils::read_lines(filename) {
      Ok(lines) => {
        for line in lines {
          if let Ok(l) = line {
            println!("raw: {l}");
            let line_decrypt = crate::crack::xor_decrypt::brute(l.as_str());
            let score = decrypt_score(line_decrypt.clone());
            let line_decrypt_string = line_decrypt.iter().map(|c| *c as char).collect::<String>();
            println!("decrypted: {}", line_decrypt_string);
            if score > res.1 {
              res = (line_decrypt_string, score);
            }
          }
        }
      }
      Err(e) => panic!("{:?}", e),
    }

    res.0
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_find_encrypted_string(){
    let res = find_encrypted_string("_data/set1/challenge4.txt");
    let expected = "Now that the party is jumping\n";
    assert_eq!(res, expected);
  }
}