use crate::program::Program;
use crate::time::ScheduleData;
use file_into_string::*;
use std::fs::File;
use std::path::Path;
use std::sync::RwLock;
use std::{env, fs};
use winrt_notification::Toast;
use windows::ApplicationModel;

use lazy_static::lazy_static;
lazy_static! {
    static ref DATA_PATH: String = "./data.json".to_string();
    pub static ref AUMID: RwLock<String> = RwLock::new("".to_string());
}

pub struct Data {}

impl Data {
    pub fn get_aumid() {
        let app_info = ApplicationModel::AppInfo::Current();

        if let Ok(ai) = app_info {
            let tst = ai.AppUserModelId().unwrap();
            let str = tst.clone().to_string();
            let mut unlocked = AUMID.write().unwrap();
            *unlocked = str;
        }

        if AUMID.read().unwrap().is_empty() {
            let mut unlocked = AUMID.write().unwrap();
            *unlocked = Toast::POWERSHELL_APP_ID.to_string();
        }

    }

    pub fn read(pr: &mut Program) {
        let exe = env::current_exe().unwrap();
        let dir = exe.parent().expect("Exe must be in some directory");
        println!("Current dir: {}", dir.to_str().unwrap());

        println!("Reading...");
        let path = Path::new(DATA_PATH.as_str());
        let file = File::open(path);
        match file {
            Ok(_) => {}
            Err(_) => {
                File::create(path).expect("Somehow failed to create the json file lol");
                Data::write(pr);
                return;
            }
        }
        let data = file_into_string(file.unwrap()).unwrap();
        let value: serde_json::Value =
            serde_json::from_str(data.as_str()).expect("Couldn't parse the json data.");
        let e: ScheduleData = serde_json::from_value(value)
            .expect("Deserialization failed when reading the json file.");
        pr.data.update(&e);
    }

    pub fn write(pr: &Program) {
        println!("Writing...");

        fs::write(
            Path::new(DATA_PATH.as_str()),
            serde_json::to_string_pretty(&pr.data).unwrap().as_str(),
        )
        .expect("Write failed");
    }

    // pub fn get_day(pr: &Program, day_type: dt) -> Result<&Day, ()> {
    //     pr.data.get_day(day_type)
    // }
}
