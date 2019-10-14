pub fn square(number: u32) -> u64 {
    // nth term in doubling sequence:
    // https://socratic.org/questions/how-do-you-find-the-nth-term-rule-for-1-2-4-8-16-32-64
    if number == 0 || number > 64 {
      panic!("Square must be between 1 and 64");
    }

    let first_term: u64 = 1;
    let ratio: u64 = 2;

    (first_term * ratio).pow(number - 1)
}

pub fn total() -> u64 {
  std::u64::MAX
}
