use std::fmt::Display;

#[derive(Debug, PartialEq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let total_minutes_24 = (hours * 60 + minutes) % 1440;
        let minutes = match total_minutes_24 >= 0 || total_minutes_24 % 60 == 0 {
            true => total_minutes_24 % 60,
            false => 60 + total_minutes_24 % 60,
        };
        let hours = match total_minutes_24 >= 0 {
            true => total_minutes_24 / 60,
            false => {
                if total_minutes_24 % 60 != 0 {
                    23 + total_minutes_24 / 60
                } else {
                    24 + total_minutes_24 / 60
                }
            }
        };
        Self { hours, minutes }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let total_minutes_24 = (self.hours * 60 + self.minutes + minutes) % 1440;
        let minutes = match total_minutes_24 >= 0 || total_minutes_24 % 60 == 0 {
            true => total_minutes_24 % 60,
            false => 60 + total_minutes_24 % 60,
        };
        let hours = match total_minutes_24 >= 0 {
            true => total_minutes_24 / 60,
            false => {
                if total_minutes_24 % 60 != 0 {
                    23 + total_minutes_24 / 60
                } else {
                    24 + total_minutes_24 / 60
                }
            }
        };
        Self { hours, minutes }
    }
}

impl Display for Clock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let hours = if self.hours < 10 {
            format!("0{}", self.hours)
        } else {
            format!("{}", self.hours)
        };
        let minutes = if self.minutes < 10 {
            format!("0{}", self.minutes)
        } else {
            format!("{}", self.minutes)
        };
        write!(f, "{}:{}", hours, minutes)
    }
}
