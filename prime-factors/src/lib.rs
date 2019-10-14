pub fn factors(n: u64) -> Vec<u64> {
    let mut factors = Vec::new();
    let mut num = n;

    while num > 1 {
        let mut range = 2..=num;
        let factor = range.find(|x| num % x == 0).unwrap();
        factors.push(factor);

        num = num / factor;
    }
    factors
}
