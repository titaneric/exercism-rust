use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}
impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let true_mins = minutes.rem_euclid(60);
        let true_hours = (hours + minutes.div_euclid(60)).rem_euclid(24);
        Self {
            hours: true_hours,
            minutes: true_mins,
        }
    }

    pub fn add_minutes(&self, new_minutes: i32) -> Self {
        let Clock { hours, minutes } = self;
        let true_mins = (minutes + new_minutes).rem_euclid(60);
        let true_hours = (hours + (minutes + new_minutes).div_euclid(60)).rem_euclid(24);
        Self {
            hours: true_hours,
            minutes: true_mins,
        }
    }
}
