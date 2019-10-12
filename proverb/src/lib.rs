pub fn build_proverb(list: &[&str]) -> String {
    let mut output = String::from("");
    let mut iter = list.iter().peekable();

    while let Some(current) = iter.next() {
      let next = iter.peek();

      let line = match next {
        Some(_x) => format!("For want of a {} the {} was lost.\n", current, next.unwrap()),
        None => format!("And all for the want of a {}.", list[0])
      };

      output = format!("{}{}", output, line)
    }

    output
}
