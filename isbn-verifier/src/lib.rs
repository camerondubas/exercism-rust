/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    if isbn.chars().filter(|c| c.is_alphanumeric()).count() != 10 {
      return false;
    }

    let mut has_invalid_char = false;
    let sum = isbn.chars().filter(|c| c.is_alphanumeric())
        .enumerate()
        .filter_map(|(idx, c)| match c {
            x if x.is_digit(10) => Some(x.to_digit(10).unwrap() * (10 - idx as u32)),
            'X' | 'x' => {
                if idx == 9 {
                    Some(10 * (10 - idx as u32))
                } else {
                    None
                }
            }
            _ => {
              has_invalid_char = true;
              None
              },
        })
        .sum::<u32>();

    if has_invalid_char {
      false
    } else {
      sum % 11 == 0
    }
}
