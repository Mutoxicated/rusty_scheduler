use std::fmt::Display;

use crate::time::pattern::Pattern;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use super::PatternInfo;

#[derive(PartialEq,Eq,Clone)]
pub enum PatternDetectionType {
    All,
    Nth(usize)
}

#[derive(std::fmt::Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
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
        f.write_fmt(format_args!("{:?}", self.day_type)).unwrap();
        Ok(())
    }
}

#[derive(Deserialize, Serialize, Clone, PartialEq, Eq, Debug)]
pub struct Day {
    pub day_type: DayType,
    pub patterns: Vec<Pattern>,
}

impl Day {
    pub fn new(day_type: DayType) -> Self {
        Self {
            day_type,
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

    pub fn from_string(string: &str) -> DayType {
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

    pub fn pattern_exists(&mut self, name:&str) -> bool {
        for pattern in &self.patterns {
            if pattern.name == *name {
                return true
            }
        }
        false
    }

    pub fn copy_pattern(&mut self, name:&str) -> Option<Pattern> {
        for pattern in &self.patterns {
            if pattern.name == name {
                return Some(pattern.clone());
            }
        }
        None
    }

    pub fn add_pattern(&mut self, pattern: Pattern) {
        self.patterns.push(pattern.clone());
        self.patterns.sort();
    }

    pub fn remove_pattern(&mut self, name: String, pdt: PatternDetectionType) {
        if pdt == PatternDetectionType::All {
            if name.is_empty() {
                self.patterns.clear();
                return;
            }
            self.patterns.retain(|x| x.name != name);
        }else {
            let mut ui = 0;
            let mut occurences = 0;
            for i in 0..self.patterns.len() {
                if self.patterns[i].name == name {
                    occurences += 1;
                    ui = i;
                    if let PatternDetectionType::Nth(x) = pdt {
                        if x == occurences {
                            self.patterns.remove(ui);
                            return;
                        }
                    }
                }
            }
            if let PatternDetectionType::Nth(x) = pdt {
                if occurences >= x || occurences == 0 {
                    return;
                }
                self.patterns.remove(ui);
            }
        }
    }

    pub fn change_pattern(&mut self, name: String, pi:&PatternInfo, pdt: PatternDetectionType) {
        let mut ui = 0;
        let mut occurences = 0;
        for i in 0..self.patterns.len() {
            if self.patterns[i].name == name {
                occurences += 1;
                ui = i;
                if let PatternDetectionType::Nth(x) = pdt {
                    if x == occurences {
                        self.patterns[ui].from(pi);
                        return;
                    }
                }else {
                    self.patterns[ui].from(pi);
                }
            }
        }
        if let PatternDetectionType::Nth(x) = pdt {
            if occurences >= x || occurences == 0 {
                return;
            }
            self.patterns[ui].from(pi);
        }
    }

    pub fn clear(&mut self){
        self.patterns.clear();
    }

    pub fn check_patterns(&mut self, time:NaiveDateTime) {
        for i in 0..self.patterns.len() {
            if self.patterns[i].is_ready(time){
                self.patterns[i].notify();
                if self.patterns[i].once {
                    self.patterns.remove(i);
                }
            }
        }
    }

    pub fn present_patterns(&self) {
        for pattern in &self.patterns {
            pattern.present();
        }
    }

    pub fn get_pattern_string(&self, idx: usize, inner_idx: usize) -> Option<String> {
        if idx >= self.patterns.len() {
            return None;
        }
        Some(self.patterns[idx].get_stringified(inner_idx))
    }
}
