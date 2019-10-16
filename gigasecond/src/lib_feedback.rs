use chrono::{DateTime, Duration, Utc};

const GIGASECOND: i64 = 1_000_000_000;

pub fn after(start: DateTime<Utc>) -> DateTime<Utc> {
    let gigasecond: Duration = Duration::seconds(GIGASECOND);

    start.checked_add_signed(gigasecond).expect("Overflow")
}
