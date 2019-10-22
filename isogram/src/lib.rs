use std::collections::HashSet;

pub fn check(candidate: &str) -> bool {
    let mut used_chars = HashSet::new();

    for c in candidate.to_lowercase().chars() {
      if c.is_alphabetic() {
        if used_chars.contains(&c) {
          return false;
        } else {
          used_chars.insert(c);
        }
      }
    }

  true
}
