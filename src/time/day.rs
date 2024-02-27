use crate::time::pattern::Pattern;
use serde_derive::{Deserialize, Serialize};

use super::pattern;

#[derive(std::fmt::Debug,Deserialize,Serialize,Clone)]
pub enum DayType{
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
    Na
}

#[derive(Deserialize,Serialize,Clone)]
pub struct Day{
    pub day_type:DayType,
    pub patterns:Vec<Pattern>
}

impl Day {
    pub fn new(day_type:DayType)->Self{
        Self {
            day_type: day_type,
            patterns: Vec::new(),
        }
    }

    pub fn from_u32(num:u32) -> DayType{
        match num {
            0 => DayType::Monday,
            1 => DayType::Tuesday,
            2 => DayType::Wednesday,
            3 => DayType::Thursday,
            4 => DayType::Friday,
            5 => DayType::Saturday,
            6 => DayType::Sunday,
            _ => DayType::Na,
        }
    }

    pub fn add_event(&mut self, event:Pattern){
        self.patterns.push(event);
    }

    pub fn check_patterns(&self,current_time:String)-> Option<&Pattern>{
        for event in &self.patterns{
            if event.is_ready(current_time.clone()){
                return Some(event);
            }
        };
        return None;
    }

    pub fn present_patterns(&self){
        for pattern in &self.patterns{
            pattern.present();
        }
    }
}