pub mod day;
mod pattern;
mod schedule;

pub use pattern::Pattern;
pub use schedule::ScheduleData;

pub struct Time {
    pub hours:i32,
    pub minutes:i32,
    pub seconds:i32,
}

impl Time {
    pub fn new(hours:i32,minutes:i32,seconds:i32) -> Self{
        Self {
            hours,
            minutes,
            seconds,
        }
    }

    pub fn print(&self){
        println!("{}:{}:{}",self.hours,self.minutes,self.seconds);
    }

    pub fn update(&mut self, formatted_time:String) {
        self.hours = formatted_time[0..2].parse().unwrap();
        self.minutes = formatted_time[3..5].parse().unwrap();
        self.seconds = formatted_time[6..8].parse().unwrap();
    }

    // pub fn tick(&mut self) -> bool{
    //     let mut min_changed = false;
    //     self.seconds += 1;

    //     if self.seconds > 59 {
    //         self.seconds = 0;
    //         self.minutes += 1;
    //         min_changed = true;
    //     }

    //     if self.minutes > 59 {
    //         self.minutes = 0;
    //         self.hours += 1;
    //     }

    //     if self.hours > 23 {
    //         self.hours = 0;
    //     }
    //     min_changed
    // }

    // pub fn tick_min(&mut self){
    //     self.minutes += 1;
    //     if self.minutes > 59 {
    //         self.minutes = 0;
    //         self.hours += 1;
    //     }

    //     if self.hours > 23 {
    //         self.hours = 0;
    //     }
    // }
}