use std::fmt;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Clock {
    minute_of_day: u16
}

const MINUTES_IN_ONE_DAY: i64 = 60 * 24;

impl Clock {
    pub fn new_from_minute_of_day(minute_of_day: i64) -> Clock {
        Clock { minute_of_day: ((minute_of_day + MINUTES_IN_ONE_DAY * 1000_000) % MINUTES_IN_ONE_DAY) as u16 }
    }

    pub fn new(hours: i64, minutes: i64) -> Clock {
        Clock::new_from_minute_of_day(hours * 60 + minutes)
    }

    pub fn add_minutes(&self, added_minutes: i64) -> Clock {
        Clock::new_from_minute_of_day(self.minute_of_day as i64 + added_minutes)
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.minute_of_day / 60, self.minute_of_day % 60)
    }
}