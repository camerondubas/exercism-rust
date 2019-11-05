#[derive(Debug, PartialEq, Eq)]
pub enum Classification {
    Abundant,
    Perfect,
    Deficient,
}

pub fn classify(num: u64) -> Option<Classification> {
    if num <= 0 {
      return None;
    }

    match aliquot_sum(num) {
      x if x > num => Some(Classification::Abundant),
      x if x == num => Some(Classification::Perfect),
      x if x < num => Some(Classification::Deficient),
      _ => None
    }
}

fn aliquot_sum(num: u64) -> u64 {
  (1..num).into_iter().filter(|i| num % i == 0).sum()
}
