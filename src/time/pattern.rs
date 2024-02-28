use serde_derive::{Deserialize, Serialize};
use crate::utils::{get_hour,get_minutes};

#[derive(Deserialize, Serialize, Clone, PartialEq, Eq, Hash, Debug,)]
pub struct Pattern {
    pub name: String,
    pub desc: String,
    pub time: String,
    pub special: Option<bool>,
}

use std::cmp::Ordering;

impl PartialOrd for Pattern {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let hour1 = get_hour(&self.time);
        let hour2 = get_hour(&other.time);
        if hour1 > hour2 {
            return Some(Ordering::Greater);
        }else if hour1 < hour2 {
            return Some(Ordering::Less);
        }
        let minutes1 = get_minutes(&self.time);
        let minutes2 = get_minutes(&other.time);
        if minutes1 > minutes2 {
            return Some(Ordering::Greater);
        }else if minutes1 < minutes2 {
            return Some(Ordering::Less);
        }else{
            return Some(Ordering::Equal);
        }
    }
}

impl Ord for Pattern {
    fn cmp(&self, other:&Self) -> Ordering {
        let hour1 = get_hour(&self.time);
        let hour2 = get_hour(&other.time);
        if hour1 > hour2 {
            return Ordering::Greater;
        }else if hour1 < hour2 {
            return Ordering::Less;
        }
        let minutes1 = get_minutes(&self.time);
        let minutes2 = get_minutes(&other.time);
        if minutes1 > minutes2 {
            return Ordering::Greater;
        }else if minutes1 < minutes2 {
            return Ordering::Less;
        }else{
            return Ordering::Equal;
        }
    }
}

impl Pattern {
    pub fn new(name: String, desc: String, time: String, special: Option<bool>) -> Self {
        Self {
            name,
            desc,
            time,
            special,
        }
    }

    pub fn new_empty() -> Self {
        Self {
            name: String::new(),
            desc: String::new(),
            time: String::new(),
            special: Some(false),
        }
    }

    pub fn is_ready(&self, current_time: String) -> bool {
        if self.time == current_time {
            true
        } else {
            false
        }
    }

    pub fn present(&self) {
        println!("|");
        if self.desc == "" {
            println!("|_ {} - {}", self.time, self.name);
        } else {
            println!("|_ {} - {} - {}", self.time, self.name, self.desc);
        }
    }
}
