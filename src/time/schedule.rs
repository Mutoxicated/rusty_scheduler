use crate::global::*;
use crate::program::{ProgramInfo, Receive};
use crate::time::day::DayType;
use crate::time::day::{Day, DayType as dt};
use crate::utils::*;
use colored::{Colorize, CustomColor};
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

    pub fn remove_pattern(&mut self, pri: &mut ProgramInfo) {}

    pub fn add_pattern(&mut self, pri: &mut ProgramInfo) {
        let args = &pri.args;

        let mut valid_days: Vec<dt> = Vec::new();
        for day in &args.days {
            let res = Day::from_string(day);
            if res == DayType::Na {
                continue;
            }
            valid_days.push(res);
        }

        if valid_days.len() == 0 {
            println!(
                "{} You didn't provide any days or the days you provided are not valid!",
                "Error!".red()
            );
            pri.finish();
            return;
        }

        pri.steps = pri.steps + 1;

        if pri.steps == 3 {
            pri.input_pattern.time = pri.input.clone();
            println!(
                "Please provide a description. {}",
                "(you can leave it empty)".custom_color(*GREY)
            );
            return;
        }
        if pri.steps == 2 {
            pri.input_pattern.name = pri.input.clone();
            println!(
                "What time? {}",
                "(please use the 24 hour format)".custom_color(*GREY)
            );
            return;
        }
        if pri.steps == 1 {
            pri.input_pattern.special = Some(yes_or_no(pri.input.clone()));
            println!("What is the name of the pattern?");
            return;
        }
        if pri.steps == 0 {
            pri.command_finished = false;
            println!("Is the event a special event?");
            return;
        }

        pri.command_finished = true;

        for valid_day in &mut valid_days {
            self.get_day(valid_day.clone())
                .unwrap()
                .add_pattern(&pri.input_pattern);
        }

        pri.finish();
        println!(
            "{} '{}' added to {:?}!",
            "Pattern".yellow(),
            pri.input_pattern.name,
            valid_days
        );
    }
}

impl Receive for ScheduleData {
    fn receive(&mut self, mut pri: &mut ProgramInfo) {
        println!("Debug: Command Name->{}", pri.command_name);

        match pri.command_name.as_str() {
            "add_pattern" => {
                self.add_pattern(&mut pri);
            }
            _ => (),
        }
    }
}
