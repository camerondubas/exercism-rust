pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut sum = 0;

    'outer: for num in 1..limit {
        'inner: for factor in factors {
            if factor == &0 {
                continue 'inner;
            }

            if num % factor == 0 {
                sum = sum + num;
                continue 'outer;
            }
        }
    }

    sum
}
