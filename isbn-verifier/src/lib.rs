/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    if isbn.chars().filter(|c| c.is_alphanumeric()).count() != 10 {
      return false;
    }

    let mut has_invalid_char = false;
    let mut sum = 0;

    for (idx, c) in isbn.chars().filter(|c| c.is_alphanumeric()).enumerate().peekable() {
      if c.is_digit(10) {
        sum = sum + c.to_digit(10).unwrap() * (10 - idx as u32);
      } else if c == 'X' && idx == 9 {
        sum = sum + 10 * (10 - idx as u32)
      } else {
        has_invalid_char = true;
        break;
      }
    }

    if has_invalid_char {
      false
    } else {
      sum % 11 == 0
    }
}
