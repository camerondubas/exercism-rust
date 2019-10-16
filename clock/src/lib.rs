use std::fmt;

#[derive(Debug)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        set_time(hours, minutes)
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        set_time(self.hours, self.minutes + minutes)
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:0>2}:{:0>2}", self.hours, self.minutes)
    }
}

impl PartialEq for Clock {
    fn eq(&self, other: &Self) -> bool {
        self.hours == other.hours && self.minutes == other.minutes
    }
}

fn set_time(hours: i32, minutes: i32) -> Clock {
    let final_minutes = minutes.rem_euclid(60);
    let rollover_minutes = minutes.div_euclid(60);
    let final_hours = (hours + rollover_minutes).rem_euclid(24);

    Clock {
        hours: final_hours,
        minutes: final_minutes,
    }
}
