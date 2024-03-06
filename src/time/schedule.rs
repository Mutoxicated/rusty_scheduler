use std::str::FromStr;

use crate::arg_parser::ArgError;
use crate::global::*;
use crate::program::{ProgramInfo, Receive};
use crate::time::day::{Day, DayType};
use crate::time::Pattern;
use crate::utils::*;
use chrono::{Local, NaiveDateTime, NaiveTime};
use colored::{Colorize, CustomColor};
use serde::{Deserialize, Serialize};

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
            monday: Day::new(DayType::Monday),
            tuesday: Day::new(DayType::Tuesday),
            wednesday: Day::new(DayType::Wednesday),
            thursday: Day::new(DayType::Thursday),
            friday: Day::new(DayType::Friday),
            saturday: Day::new(DayType::Saturday),
            sunday: Day::new(DayType::Sunday),
        }
    }

    pub fn update(&mut self, other: Self) {
        self.monday = other.monday;
        self.tuesday = other.tuesday;
        self.wednesday = other.wednesday;
        self.thursday = other.thursday;
        self.friday = other.friday;
        self.saturday = other.saturday;
        self.sunday = other.sunday;
    }

    pub fn get_day(&mut self, day_type: DayType) -> Result<&mut Day, ()> {
        match day_type {
            DayType::Monday => Ok(&mut self.monday),
            DayType::Tuesday => Ok(&mut self.tuesday),
            DayType::Wednesday => Ok(&mut self.wednesday),
            DayType::Thursday => Ok(&mut self.thursday),
            DayType::Friday => Ok(&mut self.friday),
            DayType::Saturday => Ok(&mut self.saturday),
            DayType::Sunday => Ok(&mut self.sunday),
            _ => Err(()),
        }
    }

    pub fn read_day(&self, day_type: DayType) -> Result<&Day, ()> {
        match day_type {
            DayType::Monday => Ok(&self.monday),
            DayType::Tuesday => Ok(&self.tuesday),
            DayType::Wednesday => Ok(&self.wednesday),
            DayType::Thursday => Ok(&self.thursday),
            DayType::Friday => Ok(&self.friday),
            DayType::Saturday => Ok(&self.saturday),
            DayType::Sunday => Ok(&self.sunday),
            _ => Err(()),
        }
    }

    fn get_valid_days(days: &Vec<String>)->Vec<DayType> {
        let mut vdt:Vec<DayType> = Vec::with_capacity(days.len());
        for day in days {
            if day == "all" {
                vdt = vec![
                    DayType::Monday,
                    DayType::Tuesday,
                    DayType::Wednesday,
                    DayType::Thursday,
                    DayType::Friday,
                    DayType::Saturday,
                    DayType::Sunday,
                ];
                return vdt;
            }
            let res = Day::from_string(&day.clone());
            if res == DayType::Na {
                continue;
            }
            vdt.push(res);
        }
        vdt
    }

    fn assign_valid_arguments(pri: &mut ProgramInfo){
        if let Ok(name) = &pri.args.name {
            pri.cmib.input_pattern.name = Some(name.clone());
        }
        if pri.args.all.is_some() {
            pri.cmib.all = pri.args.all;
        }
        if pri.cmib.valid_daytypes.is_none() {
            if pri.args.days.is_ok() {
                pri.cmib.valid_daytypes = Some(ScheduleData::get_valid_days(pri.args.days.as_ref().unwrap()));
            }else {
                pri.cmib.valid_daytypes = Some(Vec::new());
            }
        }
    }

    fn get_daytypes(pri:&mut ProgramInfo) -> Option<ArgError>{
        if let Err(er) = &pri.args.days {
            if pri.cmib.all.is_none() || !pri.cmib.all.unwrap() {
                return Some(er.clone())
            }else {
                pri.cmib.valid_daytypes = Some(vec![
                    DayType::Monday,
                    DayType::Tuesday,
                    DayType::Wednesday,
                    DayType::Thursday,
                    DayType::Friday,
                    DayType::Saturday,
                    DayType::Sunday,
                ]);
            }
            None
        }else {
            pri.cmib.valid_daytypes = Some(ScheduleData::get_valid_days(pri.args.days.as_ref().unwrap()));
            if pri.cmib.valid_daytypes.as_ref().unwrap().is_empty() {
                Some(ArgError::InvalidDay)
            }else {
                None
            }
        }
    }

    fn name_to_day(pri: &mut ProgramInfo)-> bool{//returns success
        if pri.cmib.input_pattern.name.is_none() {
            return false;
        }
        let dayt = Day::from_string(pri.cmib.input_pattern.name.as_ref().unwrap().as_str());
        if dayt == DayType::Na {
            return false;
        }
        pri.cmib.valid_daytypes.as_mut().unwrap().push(dayt);
        pri.cmib.input_pattern.name = None;
        true
    }

    fn name_to_all(pri: &mut ProgramInfo)->bool {
        if pri.cmib.input_pattern.name.is_none() {
            return false;
        }

        pri.cmib.all = Some(pri.cmib.input_pattern.name.as_ref().unwrap().contains("all"));
        pri.cmib.input_pattern.name = None;
        true
    }

    fn flexible_get_daytypes(pri: &mut ProgramInfo)->bool {//return failure
        if let Some(er) = ScheduleData::get_daytypes(pri){
            if !ScheduleData::name_to_day(pri) {
                ScheduleData::name_to_all(pri);
            }
            if let Some(er2) = ScheduleData::get_daytypes(pri) {
                println!("{er2}");
                pri.finish();
                return true;
            }
            if pri.cmib.valid_daytypes.as_ref().unwrap().is_empty() {
                println!("{er}");
                pri.finish();
                return true;
            }
        }
        false
    }

    fn copy_pattern(&mut self, pri: &mut ProgramInfo){
        //evaluation
        ScheduleData::assign_valid_arguments(pri);
        ScheduleData::flexible_get_daytypes(pri);

        let dayt = pri.cmib.valid_daytypes.as_ref().unwrap()[0].clone();

        if pri.cmib.input_pattern.name.is_none() {
            if !pri.asked {
                pri.asked = true;
                println!("Please provide the name of the pattern.");
                return;
            }else {
                pri.cmib.input_pattern.name = Some(pri.input.clone());
                pri.asked = false;
            }
        }

        if dayt == DayType::Na {
            println!("{}",ArgError::InvalidDay);
            pri.finish();
            return;
        }
        //execution
        let day = self.get_day(dayt.clone()).unwrap();

        if !day.pattern_exists(pri.cmib.input_pattern.name.as_ref().unwrap().as_str()){
            println!("{}",ArgError::InvalidPatternName);
        }else {
            println!("Copied {} '{}' from {dayt:?}!","Pattern".yellow(),pri.cmib.input_pattern.name.as_ref().unwrap());
            pri.pattern_copy_buffer = Some(day.copy_pattern(pri.cmib.input_pattern.name.as_ref().unwrap().as_str()).unwrap());
        }
        pri.finish();
    }

    fn paste_pattern(&mut self, pri: &mut ProgramInfo) {
        //evaluation
        if pri.pattern_copy_buffer.is_none() {
            println!("{} You didn't copy anything to paste!", "Error!".red());
            pri.finish();
            return;
        }
        ScheduleData::assign_valid_arguments(pri);
        ScheduleData::flexible_get_daytypes(pri);

        //execution
        for vd in pri.cmib.valid_daytypes.as_ref().unwrap() {
            let day = self.get_day(vd.clone());
            if let Ok(d) = day {
                d.add_pattern(pri.pattern_copy_buffer.as_ref().unwrap().clone());
            }
        }
        println!("Pasted {} '{}' onto {:?}!",
            "Pattern".yellow(),
            pri.pattern_copy_buffer.as_ref().unwrap().name,
            pri.cmib.valid_daytypes.as_ref().unwrap());
        pri.pattern_copy_buffer = None;
        pri.finish();
    }

    fn remove_pattern(&mut self, pri: &mut ProgramInfo) {
        //evaluation
        ScheduleData::assign_valid_arguments(pri);
        if let Some(er) = ScheduleData::get_daytypes(pri) {
            println!("{er}");
            pri.finish();
            return;
        }

        if pri.cmib.input_pattern.name.is_none() {
            if !pri.asked {
                pri.asked = true;
                println!("Please provide the name of the {}", "pattern".yellow());
                return;
            }else if pri.input.is_empty() {
                println!("{}", ArgError::Empty);
                pri.finish();
                return;
            }else {
                pri.cmib.input_pattern.name = Some(pri.input.clone());
                pri.asked = false;
            }
        }

        //execution

        if pri.cmib.all.is_none() {
            pri.cmib.all = Some(false);
        }

        if pri.cmib.input_pattern.name.is_none() {
            pri.cmib.input_pattern.name = Some("".to_owned());
        }

        for day in pri.cmib.valid_daytypes.as_ref().unwrap() {
            self.get_day(day.clone())
                .unwrap()
                .remove_pattern(pri.cmib.input_pattern.name.as_ref().unwrap().clone(), pri.cmib.all.unwrap());
        }

        if pri.cmib.input_pattern.name.as_ref().unwrap().is_empty() {
            println!(
                "All {} removed from {:?}!",
                "Patterns".yellow(),
                pri.cmib.valid_daytypes.as_ref().unwrap()
            );
        }else {
            println!(
                "{} '{}' removed from {:?}!",
                "Pattern".yellow(),
                pri.cmib.input_pattern.name.as_ref().unwrap(),
                pri.cmib.valid_daytypes.as_ref().unwrap()
            );
        }
        pri.finish();
    }

    fn add_pattern(&mut self, pri: &mut ProgramInfo) {
        //evaluation
        if pri.cmib.valid_daytypes.is_none() {
            ScheduleData::assign_valid_arguments(pri);
            if ScheduleData::flexible_get_daytypes(pri){
                return;
            }
        }

        if pri.cmib.input_pattern.once.is_none() {
            if !pri.asked {
                pri.asked = true;
                println!("Is the event a special event?");
                return;
            }else {
                pri.asked = false;
                pri.cmib.input_pattern.once = Some(yes_or_no(pri.input.clone()));
            }
        }
        if pri.cmib.input_pattern.name.is_none() {
            if !pri.asked {
                pri.asked = true;
                println!("Please provide the name of the pattern.");
                return;
            }else {
                pri.asked = false;
                pri.cmib.input_pattern.name = Some(pri.input.clone());
            }
        }
        if pri.cmib.input_pattern.date_time.is_none() {
            println!("Debug: asked->{}",pri.asked);
            if !pri.asked {
                pri.asked = true;
                println!(
                    "What time? {}",
                    "(please use the 24 hour format)".custom_color(*GREY)
                );
                return;
            }else {
                pri.asked = false;
                let formatted_time = format_time(pri.input.as_str());
                if let Err(x) = formatted_time {
                    println!("{x}");
                    pri.finish();
                    return;
                }
                let fmt_time_str = formatted_time.as_ref().unwrap().as_str();
                let now = Local::now();

                pri.cmib.input_pattern.date_time = Some(NaiveDateTime::new(now.date_naive(),NaiveTime::from_str(fmt_time_str).unwrap()));
            }
        }

        //execution

        for valid_day in pri.cmib.valid_daytypes.as_ref().unwrap() {
            self.get_day(valid_day.clone())
                .unwrap()
                .add_pattern(Pattern::try_from(pri.cmib.input_pattern.clone()).unwrap());
        }

        println!(
            "{} '{}' added to {:?}!",
            "Pattern".yellow(),
            pri.cmib.input_pattern.name.as_ref().unwrap(),
            pri.cmib.valid_daytypes.as_ref().unwrap()
        );
        pri.finish();
    }

    fn today(&mut self, pri: &mut ProgramInfo) {
        let day = self.get_day(pri.today.to_owned()).unwrap();
        println!("{:?}", pri.today);
        day.present_patterns();
        pri.finish();
    }

    fn get_schedule(&mut self, pri: &mut ProgramInfo) {
        ScheduleData::assign_valid_arguments(pri);
        if ScheduleData::flexible_get_daytypes(pri){
            return;
        }

        let mut day_indices: Vec<usize> = Vec::new();
        let mut actual_days: Vec<&Day> = Vec::with_capacity(pri.cmib.valid_daytypes.as_ref().unwrap().len());
        for day in pri.cmib.valid_daytypes.as_ref().unwrap() {
            actual_days.push(self.read_day(day.clone()).unwrap());
        }

        let mut line1: String = String::new();
        for day in &actual_days {
            day_indices.push(line1.len());
            line1.push_str(format!("{:?}", day.day_type).as_str());
            line1.push_str(&DAY_SPACE);
        }
        line1.push_str(&DAY_SPACE);
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

        pri.finish();
    }

    fn clear(&mut self, pri: &mut ProgramInfo) {
        ScheduleData::assign_valid_arguments(pri);

        if ScheduleData::flexible_get_daytypes(pri){
            return;
        }

        pri.cmib.valid_daytypes.as_ref().unwrap()
            .iter()
            .for_each(|d| self.get_day(d.clone()).unwrap().clear());
        
        println!("Cleared {:?}!",pri.cmib.valid_daytypes.as_ref().unwrap());

        pri.finish();
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
            "copy_pattern" => self.copy_pattern(pri),
            "paste_pattern" => self.paste_pattern(pri),
            "clear" => self.clear(pri),
            _ => {pri.command_finished = true;},
        }
    }
}
