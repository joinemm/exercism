use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Clock {
    minutes: i32,
    hours: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let mut clock = Self {
            minutes: minutes,
            hours: hours,
        };
        clock.calculate();
        clock
    }

    pub fn add_minutes(&mut self, minutes: i32) -> Self {
        self.minutes += minutes;
        self.calculate();
        Self { ..*self }
    }

    fn calculate(&mut self) {
        while self.minutes > 59 {
            self.hours += 1;
            self.minutes -= 60;
        }
        while self.minutes < 0 {
            self.hours -= 1;
            self.minutes += 60;
        }
        while self.hours > 23 {
            self.hours -= 24;
        }
        while self.hours < 0 {
            self.hours += 24;
        }
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:0>2}:{:0>2}", self.hours, self.minutes % 60,)
    }
}
