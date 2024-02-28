use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, PartialEq, Eq, Hash, Debug)]
pub struct Pattern {
    pub name: String,
    pub desc: String,
    pub time: String,
    pub special: Option<bool>,
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
