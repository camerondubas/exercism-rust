pub fn raindrops(n: u32) -> String {
    let mut output = String::new();

    let cases = [
      (3, "Pling"),
      (5, "Plang"),
      (7, "Plong"),
    ];

    for (num, word) in cases.iter() {
      if n % num == 0 {
        output.push_str(word);
      }
    }

    if output.is_empty() {
      return n.to_string()
    }

    output
}
