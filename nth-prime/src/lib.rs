pub fn nth(n: u32) -> u32 {
    let mut prime_numbers: Vec<u32> = Vec::new();

    let mut num = 2;

    while (prime_numbers.len() as u32) < n + 1 {
      if is_prime(num) {
        prime_numbers.push(num)
      }

      num = num + 1;
    }

    prime_numbers.pop().expect("Error")
}

fn is_prime(num: u32) -> bool {
  let largest_factor = num / 2;
  let lowest_factor = 2;

  let mut inc = lowest_factor;

  while inc <= largest_factor {
    if num % inc == 0 {
      return false;
    }

    inc = inc + 1;
  }

  true
}
