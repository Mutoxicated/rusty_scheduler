use colored::{ColoredString, Colorize};
use serde::{Deserialize, Serialize};
use winrt_notification::{Toast,Duration};

#[derive(Deserialize, Serialize, Clone, PartialEq, Eq, Hash, Debug)]
pub struct Pattern {
    pub name: String,
    pub desc: String,
    hours:u64,
    mins:u64,
    pub special: bool,
}

use std::cmp::Ordering;

impl PartialOrd for Pattern {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Pattern {
    fn cmp(&self, other: &Self) -> Ordering {
        let comparison = self.hours.cmp(&other.hours);
        if let Ordering::Equal = comparison {
            return self.mins.cmp(&other.mins);
        }
        comparison
    }
}

impl Pattern {
    pub fn new_empty() -> Self {
        Self {
            name: String::new(),
            desc: String::new(),
            hours:0,
            mins:0,
            special: false,
        }
    }

    pub fn change_time(&mut self,hours:u64, mins:u64){
        self.hours = hours;
        self.mins = mins;
    }

    pub fn notify(&self){
        //println!("{}","Notification!".green());
        Toast::new(Toast::POWERSHELL_APP_ID)
            .title(&self.name)
            .text1(&self.desc)
            .duration(Duration::Short)
            .show()
            .unwrap();
    }

    pub fn is_ready(&self, hours:u64, mins:u64) -> bool {
        self.hours == hours && self.mins == mins
    }

    pub fn present(&self, in_detail: bool) {
        println!("| {}{}{}", self.hours.to_string().blue(),":".blue(),self.mins.to_string().blue());
        let colored_name: ColoredString = if self.special {
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
            return format!("| {}{}{}", self.hours.to_string().blue(),":".blue(),self.mins.to_string().blue());
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
