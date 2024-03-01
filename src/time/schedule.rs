use crate::arg_parser::ArgError;
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

    pub fn read_day(&self, day_type: dt) -> Result<&Day, ()> {
        match day_type {
            dt::Monday => Ok(&self.monday),
            dt::Tuesday => Ok(&self.tuesday),
            dt::Wednesday => Ok(&self.wednesday),
            dt::Thursday => Ok(&self.thursday),
            dt::Friday => Ok(&self.friday),
            dt::Saturday => Ok(&self.saturday),
            dt::Sunday => Ok(&self.sunday),
            _ => Err(()),
        }
    }

    fn get_valid_days(valid_days: &mut Vec<dt>, days: &Vec<String>) {
        for day in days {
            let res = Day::from_string(&day.clone());
            if res == DayType::Na {
                continue;
            }
            valid_days.push(res);
        }
    }

    fn remove_pattern(&mut self, pri: &mut ProgramInfo) {
        if let Err(er) = &pri.args.days {
            println!("{er}");
            pri.finish();
            return;
        }
        let mut valid_days: Vec<dt> = Vec::new();
        ScheduleData::get_valid_days(&mut valid_days, pri.args.days.as_ref().unwrap());

        pri.steps += 1;

        if pri.input.is_empty() && pri.steps == 1 {
            println!("{}", ArgError::Empty);
            pri.finish();
            return;
        }
        if pri.args.name.is_err() {
            if pri.steps == 0 {
                println!("Please provide the name of the {}", "pattern".yellow());
                return;
            } else if pri.steps == 1 {
                pri.input_pattern.name = pri.input.clone();
                println!("Remove all the patterns with that name?");
            }
        } else {
            pri.input_pattern.name = pri.args.name.as_ref().unwrap().to_owned();
        }
        pri.args.all = yes_or_no(pri.input.clone());

        for day in &valid_days {
            self.get_day(day.clone())
                .unwrap()
                .remove_pattern(pri.input_pattern.name.clone(), pri.args.all);
        }

        println!(
            "{} '{}' removed from {valid_days:?}!",
            "Pattern".yellow(),
            pri.input_pattern.name
        );
        pri.finish();
    }

    fn add_pattern(&mut self, pri: &mut ProgramInfo) {
        if let Err(er) = &pri.args.days {
            if pri.args.name.is_err() {
                println!("{er}");
                pri.finish();
                return;
            }
        }
        let mut valid_days: Vec<dt> = Vec::new();
        if let Ok(dayt) = &pri.args.name {
            valid_days.push(Day::from_string(dayt));
            if valid_days[0] == DayType::Na {
                println!("{}",ArgError::InvalidDay);
                pri.finish();
                return;
            }
        } else {
            ScheduleData::get_valid_days(&mut valid_days, pri.args.days.as_ref().unwrap());
        }

        if valid_days.is_empty() {
            println!("{}", ArgError::DayFormat);
            pri.finish();
            return;
        }

        pri.steps += 1;
        if pri.steps == 4 {
            pri.input_pattern.desc = pri.input.clone();
        }
        if pri.steps == 3 {
            let formatted_time = format_time(pri.input.as_str());
            if let Err(x) = formatted_time {
                println!("{x}");
                pri.finish();
                return;
            }
            pri.input_pattern.change_time(formatted_time.unwrap().as_str());
            println!(
                "Please provide a description. {}",
                "(you can leave it empty)".custom_color(*GREY)
            );
            return;
        }
        if pri.steps == 2 {
            if pri.input.is_empty() {
                println!("{}", ArgError::Empty);
                pri.finish();
                return;
            }
            pri.input_pattern.name = pri.input.clone();
            println!(
                "What time? {}",
                "(please use the 24 hour format)".custom_color(*GREY)
            );
            return;
        }
        if pri.steps == 1 {
            pri.input_pattern.special = yes_or_no(pri.input.clone());
            println!("What is the name of the pattern?");
            return;
        }
        if pri.steps == 0 {
            pri.command_finished = false;
            println!("Is the event a special event?");
            return;
        }

        for valid_day in &valid_days {
            self.get_day(valid_day.clone())
                .unwrap()
                .add_pattern(&pri.input_pattern);
        }

        println!(
            "{} '{}' added to {:?}!",
            "Pattern".yellow(),
            pri.input_pattern.name,
            valid_days
        );
        pri.finish();
    }

    fn today(&mut self, pri: &mut ProgramInfo) {
        let day = self.get_day(pri.today.to_owned()).unwrap();
        println!("{:?}", pri.today);
        day.present_patterns(true);
        pri.finish();
    }

    fn get_schedule(&mut self, pri: &mut ProgramInfo) {
        let mut valid_daytypes: Vec<dt> = Vec::new();
        if let Err(arg_err) = &pri.args.days {
            if let Ok(name) = &pri.args.name {
                valid_daytypes.push(Day::from_string(name));
                if valid_daytypes[0] == DayType::Na {
                    println!("{}",ArgError::InvalidDay);
                    pri.finish();
                    return;
                }
            }else{
                if *arg_err == ArgError::DayFormat {
                    println!("{arg_err}");
                    pri.finish();
                    return;
                }
                valid_daytypes = vec![
                    dt::Monday,
                    dt::Tuesday,
                    dt::Wednesday,
                    dt::Thursday,
                    dt::Friday,
                    dt::Saturday,
                    dt::Sunday,
                ];
            }
        } else {
            ScheduleData::get_valid_days(&mut valid_daytypes, pri.args.days.as_ref().unwrap());
            if valid_daytypes.is_empty() {
                println!("{}", ArgError::InvalidDay);
                pri.finish();
                return;
            }
        }
        self.present_schedule(valid_daytypes);
        pri.finish();
    }

    fn present_schedule(&mut self, days: Vec<dt>) {
        let mut day_indices: Vec<usize> = Vec::new();
        let mut actual_days: Vec<&Day> = Vec::new();
        for day in &days {
            actual_days.push(self.read_day(day.clone()).unwrap());
        }

        let mut line1: String = String::new();
        for day in &actual_days {
            day_indices.push(line1.len());
            line1.push_str(format!("{:?}", day.day_type).as_str());
            line1.push_str(&DAY_SPACE);
        }
        println!("{line1}");

        let width = line1.len();
        let mut mold_line: String = String::new();
        for _ in 0..width {
            mold_line.push(' ');
        }

        let mut curr_pat: usize = 0;
        let mut curr_pat_inner: usize = 0;
        let mut current_pattern_line: Option<String>;
        let mut succesful_pattern_lines: Vec<bool> = Vec::new();

        loop {
            let mut curr_line = mold_line.clone();

            if curr_pat_inner > 1 {
                curr_pat += 1;
                curr_pat_inner = 0
            }

            for i in 0..actual_days.len() {
                current_pattern_line = actual_days[i].get_pattern_string(curr_pat, curr_pat_inner);
                if let Some(line) = current_pattern_line {
                    succesful_pattern_lines.push(true);
                    curr_line.replace_range(
                        day_indices[i]
                            ..day_indices[i] + line.len(),
                            line.as_str(),
                    );
                }
            }

            if succesful_pattern_lines.is_empty() {
                break;
            }

            curr_pat_inner += 1;
            println!("{curr_line}");
            succesful_pattern_lines.clear();
        }
    }
}

impl Receive for ScheduleData {
    fn receive(&mut self, pri: &mut ProgramInfo) {
        //println!("Debug: Command Name->{}", pri.command_name);
        if pri.command_finished {
            println!("{}", "|".custom_color(*GREY));
        }

        match pri.command_name.as_str() {
            "add_pattern" => self.add_pattern(pri),
            "remove_pattern" => self.remove_pattern(pri),
            "today" => self.today(pri),
            "get_schedule" => self.get_schedule(pri),
            _ => (),
        }
    }
}
