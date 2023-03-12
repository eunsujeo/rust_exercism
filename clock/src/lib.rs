use std::fmt;

#[derive(Debug)]
pub struct Clock {
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let total_minutes = ((hours * 60 + minutes) % (24 * 60) + 24 * 60) % (24 * 60);
        Clock { minutes: total_minutes }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock::new(0, self.minutes + minutes)
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let hours = self.minutes / 60;
        let minutes = self.minutes % 60;
        write!(f, "{:02}:{:02}", hours, minutes)
    }
}

impl PartialEq for Clock {
    fn eq(&self, other: &Self) -> bool {
        self.minutes == other.minutes
    }
}

