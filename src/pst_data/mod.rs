use crate::time::day::Day;
use crate::time::day::DayType as dt;
use serde_derive::{Deserialize, Serialize};
use serde_json;
use std::fs;
use std::fs::File;
use file_into_string::*;
use std::path::Path;

use lazy_static::lazy_static;

lazy_static!{
    static ref DATA_PATH:String = "./src/data.json".to_string();
}

#[derive(Deserialize, Serialize,Clone)]
pub struct ScheduleData {
    monday:Day,
    tuesday:Day,
    wednesday:Day,
    thursday:Day,
    friday:Day,
    saturday:Day,
    sunday:Day,
}

impl ScheduleData {
    pub fn new()-> Self{
        Self{
            monday:Day::new(dt::Monday),
            tuesday:Day::new(dt::Tuesday),
            wednesday:Day::new(dt::Wednesday),
            thursday:Day::new(dt::Thursday),
            friday:Day::new(dt::Friday),
            saturday:Day::new(dt::Saturday),
            sunday:Day::new(dt::Sunday)
        }
    }

    pub fn update(&mut self, other:&Self){
        self.monday = other.monday.clone();
        self.tuesday = other.tuesday.clone();
        self.wednesday = other.wednesday.clone();
        self.thursday = other.thursday.clone();
        self.friday = other.friday.clone();
        self.saturday = other.saturday.clone();
        self.sunday = other.sunday.clone();
    }
}

use crate::program::Program;

pub struct Data {}

impl Data {
    pub fn read(pr:&mut Program){
        println!("Reading...");
        let path = Path::new(DATA_PATH.as_str());
        let file = File::open(path);
        match file {
            Ok(_) => {},
            Err(_) => {
                File::create(path).expect("Somehow failed to create the json file lol");
                Data::write(&pr);
                return ()
            }
        }
        let data = file_into_string(file.unwrap()).unwrap();
        let value:serde_json::Value = serde_json::from_str(data.as_str()).expect("Couldn't parse the json data.");
        let e:ScheduleData = serde_json::from_value(value).expect("Deserialization failed when reading the json file.");
        pr.data.update(&e);
    }
    
    pub fn write(pr:&Program){
        println!("Writing...");

        fs::write(Path::new(DATA_PATH.as_str()),serde_json::to_string_pretty(&pr.data).unwrap().as_str()).expect("Write failed");
    }

    pub fn get_day(pr:&Program,day_type:dt)->Result<Day,()>{
        let data = &pr.data;
        match day_type {
            dt::Monday => Ok(data.monday.clone()),
            dt::Tuesday => Ok(data.tuesday.clone()),
            dt::Wednesday => Ok(data.wednesday.clone()),
            dt::Thursday => Ok(data.thursday.clone()),
            dt::Friday => Ok(data.friday.clone()),
            dt::Saturday => Ok(data.saturday.clone()),
            dt::Sunday => Ok(data.sunday.clone()),
            _ => Err(())
        }
    }

    pub fn update_day(pr:&mut Program,day:Day){
        let data = &mut pr.data;
        match day.day_type {
            dt::Monday => data.monday = day,
            dt::Tuesday => data.tuesday = day,
            dt::Wednesday => data.wednesday = day,
            dt::Thursday => data.thursday = day,
            dt::Friday => data.friday = day,
            dt::Saturday => data.saturday = day,
            dt::Sunday => data.sunday = day,
            _ => ()
        }
    }
}