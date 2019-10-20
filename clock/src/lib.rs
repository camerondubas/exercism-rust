use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let final_minutes = minutes.rem_euclid(60);
        let rollover_minutes = minutes.div_euclid(60);
        let final_hours = (hours + rollover_minutes).rem_euclid(24);

        Clock {
            hours: final_hours,
            minutes: final_minutes,
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock::new(self.hours, self.minutes + minutes)
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:0>2}:{:0>2}", self.hours, self.minutes)
    }
}


impl From<Clock> for String {
    fn from(s: Clock) -> String {
        s.to_string()
    }
}
