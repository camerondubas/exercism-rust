pub fn is_armstrong_number(num: u32) -> bool {
    let digits = digits_from_number(num);
    let length = digits.len() as u32;
    let sum = digits.iter().fold(0, |acc, digit| acc + digit.pow(length));

    sum == num
}


fn digits_from_number(num: u32) -> Vec<u32> {
    let mut digits = Vec::new();
    let mut num = num;

    while num > 9 {
      digits.push(num % 10);
      num = num / 10;
    }

    digits.push(num);
    digits.reverse();
    digits
}
