pub fn square(number: u32) -> u64 {
    if number == 0 || number > 64 {
      panic!("Square must be between 1 and 64");
    }

    let first_term: u64 = 1;
    let ratio: u64 = 2;

    let a = (first_term * ratio).pow(number - 1);

    println!("{}", a);

    a
}

pub fn total() -> u64 {
  // let two: u64 = 2;
  // let squares: u32 = 64;
  // two.pow(squares)
  std::u64::MAX
}
