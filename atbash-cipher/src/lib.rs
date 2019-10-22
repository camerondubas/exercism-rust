use std::char;

/// "Encipher" with the Atbash cipher.
pub fn encode(plain: &str) -> String {
   add_spaces(apply_cipher(plain))
}

/// "Decipher" with the Atbash cipher.
pub fn decode(cipher: &str) -> String {
  apply_cipher(cipher)
}


fn add_spaces(string: String) -> String {
  let chars: Vec<char> = string.chars().collect();

  chars
    .chunks(5)
    .map(|chunk| chunk.iter().collect::<String>() + " ")
    .collect::<String>()
    .trim_end().to_string()
}

fn apply_cipher(input: &str) -> String {
  input.to_lowercase().chars().filter(|c| {
    match c {
     x if x.is_ascii_alphanumeric() => true,
      _ => false
    }
  })
  .map(|c| {
    match c {
      x if x.is_ascii_alphabetic() => {
        let digit = 26 - (c.to_digit(36).unwrap() - 10);
        char::from_digit(digit + 9, 36).unwrap()
      },
      _ => c
    }
  })
  .collect::<String>()
}
