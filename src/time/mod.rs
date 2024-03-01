pub mod day;
mod pattern;
mod schedule;

pub use pattern::Pattern;
pub use schedule::ScheduleData;

pub struct Time {
    pub hours:i32,
    pub minutes:i32,
    pub seconds:i32
}

impl Time {
    pub fn new(hours:i32,minutes:i32,seconds:i32) -> Self{
        Self {
            hours,
            minutes,
            seconds
        }
    }

    pub fn tick_min(&mut self){
        self.minutes += 1;
        if self.minutes > 59 {
            self.minutes = 0;
            self.hours += 1;
        }else{
            return
        }

        if self.hours > 23 {
            self.hours = 0;
        }
    }
}