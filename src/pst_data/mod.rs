use crate::program::Program;
use crate::time::ScheduleData;
use file_into_string::*;
use std::fs::File;
use std::path::Path;
use std::{env, fs};
use std::sync::{Mutex,Arc};

use lazy_static::lazy_static;
lazy_static! {
    static ref DATA_PATH: String = "./data.json".to_string();
}

pub struct Data {
    pub pr:Arc<Mutex<Program>>,
}

impl Drop for Data {
    fn drop(&mut self) {
        self.write();
    }
}

impl Data {
    pub fn read(&mut self) {
        let exe = env::current_exe().unwrap();
        let dir = exe.parent().expect("Exe must be in some directory");
        println!("Current dir: {}", dir.to_str().unwrap());

        println!("Reading...");
        let path =DATA_PATH.as_str();
        let file = File::open(path);
        match file {
            Ok(_) => {}
            Err(_) => {
                File::create(path).expect("Somehow failed to create the json file lol");
                self.write();
                return;
            }
        }
        let data = file_into_string(file.unwrap()).unwrap();
        let value: ScheduleData = serde_json::from_str(data.as_str()).expect("Couldn't parse the json data.");
        self.pr.as_ref().lock().unwrap().data.update(value);
    }

    pub fn write(&self) {
        println!("Writing...");

        fs::write(
            DATA_PATH.as_str(),
            serde_json::to_string_pretty(&self.pr.lock().unwrap().data).unwrap(),
        )
        .expect("Write failed");
    }

    // pub fn get_day(pr: &Program, day_type: dt) -> Result<&Day, ()> {
    //     pr.data.get_day(day_type)
    // }
}
