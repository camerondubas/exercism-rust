use chrono::{DateTime, Duration, Utc};

// Returns a Utc DateTime one billion seconds after start.
pub fn after(start: DateTime<Utc>) -> DateTime<Utc> {
    let gigasecond: Duration = Duration::seconds(1000000000);

    match start.checked_add_signed(gigasecond) {
        Some(x) => x,
        None => {
            eprintln!("Time overflows");
            start
        }
    }
}
