use chrono::{NaiveDateTime, Timelike};
use colored::{ColoredString, Colorize};
use serde::{Deserialize, Serialize};
use winrt_notification::{Toast,Duration};
use std::cmp::Ordering;

use crate::limit_to;
#[derive(Default,Clone,PartialEq, Eq)]
pub enum PatternInfoType<T> {
    New(T),
    Old,
    #[default]
    None
}

impl<T> PatternInfoType<T> {
    pub fn unwrap(self)->T{
        match self {
            PatternInfoType::New(x) => x,
            _ => {
                panic!("Failed to unwrap PatternInfoType value")
            }
        }
    }

    pub fn as_ref(&self) -> Option<&T> {
        match *self {
            PatternInfoType::New(ref x) => Some(x),
            _ => None,
        }
    }

    // pub fn is_new(&self)->bool {
    //     matches!(*self,PatternInfoType::New(_))
    // }

    // pub fn is_old(&self)->bool {
    //     matches!(*self,PatternInfoType::Old)
    // }

    pub fn is_none(&self)->bool{
        matches!(*self,PatternInfoType::None)
    }
}

#[derive(Default,Clone)]
pub struct PatternInfo {
    pub name: PatternInfoType<String>,
    pub date_time:PatternInfoType<NaiveDateTime>,
    pub once: PatternInfoType<bool>,
}

#[derive(Deserialize, Serialize, Clone, PartialEq, Eq, Debug)]
pub struct Pattern {
    pub name: String,
    pub date_time:NaiveDateTime,
    pub once: bool,
    pub mandatory: bool
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
            mandatory:false
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

    pub fn from(&mut self,other:&PatternInfo){
        println!("from");
        if let PatternInfoType::New(value) = &other.name {
            self.name = value.clone();
        }
        if let PatternInfoType::New(value) = &other.date_time {
            println!("new time");
            self.date_time = *value;
        }
        if let PatternInfoType::New(value) = &other.once {
            self.once = *value;
        }
    }

    pub fn change_once(&mut self, new_val:bool){
        if self.mandatory {
            return;
        }
        self.once = new_val;
    }

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
            format!("= {}",limit_to(self.date_time.time().to_string(),5))
        }else {
            format!("- {}", self.name)
        }
    }
}
