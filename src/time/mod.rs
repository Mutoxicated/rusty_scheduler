pub mod day;
mod pattern;
mod schedule;

use std::fmt::Display;

pub use pattern::Pattern;
pub use schedule::ScheduleData;

pub struct Time {
    pub hours:u64,
    pub minutes:u64,
    pub seconds:u64,
}

impl Display for Time {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"{}:{}:{}",self.hours,self.minutes,self.seconds)
    }
}

impl Time {
    pub fn new(hours:u64,minutes:u64,seconds:u64) -> Self{
        Self {
            hours,
            minutes,
            seconds,
        }
    }

    pub fn update(&mut self, hours:u32, minutes:u32, seconds:u32)
    {        
        self.hours = hours.into();
        self.minutes = minutes.into();
        self.seconds = seconds.into();
    }
}