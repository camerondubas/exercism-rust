pub fn square_of_sum(n: u32) -> u32 {
    let mut sum: u32 = 0;

    for num in 0..n + 1 {
     sum = sum + num;
    }

    sum.pow(2)
}

pub fn sum_of_squares(n: u32) -> u32 {
    let mut sum: u32 = 0;

    for num in 0..n + 1 {
     sum = sum + num.pow(2);
    }

    sum
}

pub fn difference(n: u32) -> u32 {
    square_of_sum(n) - sum_of_squares(n)
}
