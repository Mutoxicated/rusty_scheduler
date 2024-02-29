use crate::utils::{get_hour, get_minutes};
use colored::{ColoredString, Colorize};
use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, PartialEq, Eq, Hash, Debug)]
pub struct Pattern {
    pub name: String,
    pub desc: String,
    pub time: String,
    pub special: Option<bool>,
}

use std::cmp::Ordering;

impl PartialOrd for Pattern {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Pattern {
    fn cmp(&self, other: &Self) -> Ordering {
        let hour1 = get_hour(&self.time);
        let hour2 = get_hour(&other.time);
        if hour1 > hour2 {
            return Ordering::Greater;
        } else if hour1 < hour2 {
            return Ordering::Less;
        }
        let minutes1 = get_minutes(&self.time);
        let minutes2 = get_minutes(&other.time);
        if minutes1 > minutes2 {
            Ordering::Greater
        } else if minutes1 < minutes2 {
            Ordering::Less
        } else {
           Ordering::Equal
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
        self.time == current_time
    }

    pub fn present(&self, in_detail: bool) {
        println!("| {}", self.time.blue());
        let colored_name: ColoredString = if self.special.unwrap() {
            self.name.green()
        } else {
            self.name.yellow()
        };
        if self.desc.is_empty() || !in_detail {
            println!("|_ {}", colored_name);
        } else {
            println!("|_ {} - {}", colored_name, self.desc);
        }
    }

    pub fn get_stringified(&self, idx: usize, in_detail: bool) -> String {
        if idx == 0 {
            return format!("| {}", self.time);
        }
        // let colored_name:ColoredString;
        // if self.special.unwrap() {
        //     colored_name = self.name.green();
        // }else{
        //     colored_name = self.name.yellow();
        // }
        if self.desc.is_empty() || !in_detail {
            format!("|_ {}", self.name)
        } else {
            format!("|_ {} - {}", self.name, self.desc)
        }
    }
}
