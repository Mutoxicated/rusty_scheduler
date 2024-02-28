use std::fmt::Display;

use crate::time::pattern::Pattern;
use serde_derive::{Deserialize, Serialize};

#[derive(std::fmt::Debug, Deserialize, Serialize, Clone, PartialEq, Hash, Eq)]
pub enum DayType {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
    Na,
}

impl Display for Day {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:?}",self.day_type)).unwrap();
        Ok(())
    }
}

#[derive(Deserialize, Serialize, Clone, PartialEq, Eq, Hash, Debug)]
pub struct Day {
    pub day_type: DayType,
    pub patterns: Vec<Pattern>,
}

impl Day {
    pub fn new(day_type: DayType) -> Self {
        Self {
            day_type: day_type,
            patterns: Vec::new(),
        }
    }

    pub fn from_u32(num: u32) -> DayType {
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

    pub fn from_string(string: &String) -> DayType {
        let mut max = string.len();
        if max > 3 {
            max = 3
        }
        let generalized_string = &string.trim().to_lowercase()[0..max];
        //println!("{}",generalized_string);
        match generalized_string {
            "mon" => DayType::Monday,
            "tue" => DayType::Tuesday,
            "wed" => DayType::Wednesday,
            "thu" => DayType::Thursday,
            "fri" => DayType::Friday,
            "sat" => DayType::Saturday,
            "sun" => DayType::Sunday,
            _ => DayType::Na,
        }
    }

    fn pattern_name_exists(&mut self, name: &String) -> bool {
        for pattern in &self.patterns {
            if pattern.name == *name {
                return true;
            }
        }
        false
    }

    pub fn add_pattern(&mut self, pattern: &Pattern) {
        self.patterns.push(pattern.clone());
    }

    pub fn remove_pattern(&mut self, name: String, all: bool) {
        for i in 0..self.patterns.len() {
            if self.patterns[i].name == name {
                self.patterns.remove(i);
                if !all {
                    return ();
                }
            }
        }
    }

    pub fn check_patterns(&self, current_time: String) -> Option<&Pattern> {
        for event in &self.patterns {
            if event.is_ready(current_time.clone()) {
                return Some(event);
            }
        }
        return None;
    }

    pub fn present_patterns(&self) {
        for pattern in &self.patterns {
            pattern.present();
        }
    }
}
