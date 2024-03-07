use chrono::{NaiveDateTime, Timelike};
use colored::{ColoredString, Colorize};
use serde::{Deserialize, Serialize};
use winrt_notification::{Toast,Duration};
use std::cmp::Ordering;

use crate::limit_to;

#[derive(Default,Clone)]
pub struct PatternInfo {
    pub name: Option<String>,
    pub date_time:Option<NaiveDateTime>,
    pub once: Option<bool>,
}

#[derive(Deserialize, Serialize, Clone, PartialEq, Eq, Debug)]
pub struct Pattern {
    pub name: String,
    pub date_time:NaiveDateTime,
    pub once: bool,
}

impl TryFrom<PatternInfo> for Pattern {
    type Error = ();

    fn try_from(value: PatternInfo) -> Result<Self, Self::Error> {
        if value.name.is_none() || value.date_time.is_none() || value.once.is_none() {
            return Err(())
        }
        Ok(Self {
            name:value.name.unwrap(),
            date_time:value.date_time.unwrap(),
            once:value.once.unwrap(),
        })
    }
}

impl PartialOrd for Pattern {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Pattern {
    fn cmp(&self, other: &Self) -> Ordering {
        let comp = self.date_time.hour().cmp(&other.date_time.hour());
        if comp.is_eq() {
            return self.date_time.minute().cmp(&other.date_time.minute())
        }
        comp
    }
}

impl Pattern {
    // pub fn new_empty() -> Self {
    //     Self {
    //         name: String::new(),
    //         date_time:NaiveDateTime::MAX,
    //         once: false,
    //     }
    // }

    fn cmp_date_time(&self, other: &NaiveDateTime) -> Ordering {
        let comp = self.date_time.hour().cmp(&other.hour());
        if comp.is_eq() {
            return self.date_time.minute().cmp(&other.minute())
        }
        comp
    }
    
    pub fn notify(&self){
        //println!("{}","Notification!".green());
        Toast::new(Toast::POWERSHELL_APP_ID)
            .title(&self.name)
            .text1("Called from Rusty Scheduler.")
            .duration(Duration::Short)
            .show()
            .unwrap();
    }

    pub fn is_ready(&self, date_time:NaiveDateTime) -> bool {
        self.cmp_date_time(&date_time) == Ordering::Equal
    }

    pub fn present(&self) {
        println!("= {}",limit_to(self.date_time.time().to_string(),5).blue());
        let colored_name: ColoredString = if self.once {
            self.name.green()
        } else {
            self.name.yellow()
        };
        println!("- {}", colored_name);
    }

    pub fn get_stringified(&self, idx: usize) -> String {
        if idx == 0 {
            format!(" {}",limit_to(self.date_time.time().to_string(),5))
        }else {
            format!("- {}", self.name)
        }
    }
}
