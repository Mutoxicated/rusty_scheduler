use serde_derive::{Deserialize, Serialize};
//use crate::global;

#[derive(Deserialize,Serialize,Clone)]
pub struct Pattern {
    pub name:String,
    pub desc:String,
    pub time:String,
}

impl Pattern {
    pub fn new(name:String, desc:String,time:String) -> Self{
        Self {
            name,
            desc,
            time,
        }
    }

    pub fn is_ready(&self, current_time:String) -> bool{
        if self.time == current_time {
            true
        }else{
            false
        }
    }

    pub fn present(&self){
        println!("|");
        println!("|_ {} - {} - {}",self.time,self.name,self.desc);
        println!("");
    }
}