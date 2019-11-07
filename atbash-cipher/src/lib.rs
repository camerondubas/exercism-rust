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
  string
    .chars()
    .collect::<Vec<char>>()
    .chunks(5)
    .map(|chunk| chunk.iter().collect::<String>())
    .collect::<Vec<String>>()
    .join(" ")
}

fn apply_cipher(input: &str) -> String {
  input
  .to_lowercase()
  .chars()
  .filter(|c| c.is_ascii_alphanumeric())
  .map(|c| {
    match c {
      x if x.is_ascii_alphabetic() => {
        (26 - ((x as u8) - 97) + 96) as char
      },
      _ => c
    }
  })
  .collect::<String>()
}
