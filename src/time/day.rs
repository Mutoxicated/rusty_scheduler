use std::{fmt::Display, str::FromStr};

use crate::time::pattern::Pattern;
use chrono::{Local, NaiveDateTime, NaiveTime};
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
            patterns: vec![
                Pattern {
                    name:"Sleep".to_owned(),
                    date_time: NaiveDateTime::new(Local::now().date_naive(),NaiveTime::from_str("23:30").unwrap()),
                    once: false,
                    mandatory: true
                }
            ],
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

    fn remove_pat(&mut self, index:usize){
        if !self.patterns[index].mandatory {
            self.patterns.remove(index);
        }
    }

    pub fn remove_pattern(&mut self, name: String, pdt: PatternDetectionType) {
        let mut ui = 0;
        let mut occurences = 0;
        for i in 0..self.patterns.len() {
            if self.patterns[i].name == name {
                occurences += 1;
                ui = i;
                if let PatternDetectionType::Nth(x) = pdt {
                    if x == occurences {
                        self.remove_pat(ui);
                        return;
                    }
                }else {
                    self.remove_pat(i)
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

    fn change_pat(&mut self, index:usize, pi: &PatternInfo){
        if !self.patterns[index].mandatory {
            self.patterns[index].from(pi);
        }else {
            let oldname = self.patterns[index].name.clone();
            self.patterns[index].from(pi);
            self.patterns[index].name = oldname;
            self.patterns[index].once = false;
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
                        self.change_pat(ui,pi);
                        return;
                    }
                }else {
                    self.change_pat(i,pi);
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
