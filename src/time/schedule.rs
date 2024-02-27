use crate::program::Receive;
use crate::time::day::{Day, DayType as dt};
use crate::time::pattern::Pattern;
use crate::utils::*;
use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
pub struct ScheduleData {
    pub monday: Day,
    pub tuesday: Day,
    pub wednesday: Day,
    pub thursday: Day,
    pub friday: Day,
    pub saturday: Day,
    pub sunday: Day,
}

impl ScheduleData {
    pub fn new() -> Self {
        Self {
            monday: Day::new(dt::Monday),
            tuesday: Day::new(dt::Tuesday),
            wednesday: Day::new(dt::Wednesday),
            thursday: Day::new(dt::Thursday),
            friday: Day::new(dt::Friday),
            saturday: Day::new(dt::Saturday),
            sunday: Day::new(dt::Sunday),
        }
    }

    pub fn update(&mut self, other: &Self) {
        self.monday = other.monday.clone();
        self.tuesday = other.tuesday.clone();
        self.wednesday = other.wednesday.clone();
        self.thursday = other.thursday.clone();
        self.friday = other.friday.clone();
        self.saturday = other.saturday.clone();
        self.sunday = other.sunday.clone();
    }

    pub fn get_day(&mut self, day_type: dt) -> Result<&mut Day, ()> {
        match day_type {
            dt::Monday => Ok(&mut self.monday),
            dt::Tuesday => Ok(&mut self.tuesday),
            dt::Wednesday => Ok(&mut self.wednesday),
            dt::Thursday => Ok(&mut self.thursday),
            dt::Friday => Ok(&mut self.friday),
            dt::Saturday => Ok(&mut self.saturday),
            dt::Sunday => Ok(&mut self.sunday),
            _ => Err(()),
        }
    }

    pub fn get_day_string(&mut self, day_type: String) -> Result<&mut Day, ()> {
        let generalized_string = &day_type.trim().to_lowercase()[0..3];
        match generalized_string {
            "mon" => Ok(&mut self.monday),
            "tue" => Ok(&mut self.tuesday),
            "wed" => Ok(&mut self.wednesday),
            "thu" => Ok(&mut self.thursday),
            "fri" => Ok(&mut self.friday),
            "sat" => Ok(&mut self.saturday),
            "sun" => Ok(&mut self.sunday),
            _ => Err(()),
        }
    }

    pub fn add_pattern(&mut self, days: &mut Vec<String>) {
        if days.len() == 0 {
            let input = assign_str_from_input("Please give 1 or multiple day(s).");
            *days = parse_arguments(&input);
        }

        let mut pattern: Pattern = Pattern::new_empty();
        pattern.name = assign_str_from_input("What is the name of the event?");
        pattern.time = assign_str_from_input("What time is the event gonna happen?");
        pattern.special = yes_or_no("Is it a special event?");
        let answer = yes_or_no("Would you like to provide a description for the event?");
        if answer {
            pattern.desc = assign_str_from_input("Please provide a description.");
        }

        let mut valid_days: Vec<dt> = Vec::new();

        for day in days {
            let res = self.get_day_string(day.clone());
            if let Err(_) = res {
                continue;
            }
            valid_days.push(Day::from_string(day.clone()));
            res.unwrap().add_pattern(&pattern);
        }

        println!("Event '{}' added to {:?}!", pattern.name, valid_days);
    }

    // pub fn update_day(&mut self,day:Day){
    //     match day.day_type {
    //         dt::Monday => self.monday = day,
    //         dt::Tuesday => self.tuesday = day,
    //         dt::Wednesday => self.wednesday = day,
    //         dt::Thursday => self.thursday = day,
    //         dt::Friday => self.friday = day,
    //         dt::Saturday => self.saturday = day,
    //         dt::Sunday => self.sunday = day,
    //         _ => (),
    //     }
    // }
}

impl Receive for ScheduleData {
    fn receive(&mut self, input: String) {
        let mut parameterless_command: String = input.clone();
        let mut parameters = String::new();
        let index_at_space = parameterless_command.find(" ");
        if let Some(i) = index_at_space {
            parameters = input[i + 1..input.len()].to_string();
            parameterless_command.replace_range(i..input.len(), "");
        }
        let mut args = parse_arguments(&parameters);

        //println!("{:?}",args);

        match parameterless_command.as_str() {
            "add_pattern" => {
                self.add_pattern(&mut args);
            }
            _ => (),
        }
    }
}
