use crate::arg_parser::Args;
use crate::global::*;
use crate::pst_data::Data;
use crate::time::day::DayType;
use crate::time::Pattern;
use crate::time::ScheduleData;
use crate::utils::format_command_name;

use colored::Colorize;

pub trait Receive {
    fn receive(&mut self, pr: &mut ProgramInfo);
}

pub struct ProgramInfo {
    pub command_name: String,
    pub args: Args,
    pub input_pattern: Pattern,
    pub pattern_copy_buffer: Option<Pattern>,
    pub command_finished: bool,
    pub input: String,
    pub steps: i32,
    pub today: DayType,
}

impl ProgramInfo {
    pub fn new() -> Self {
        Self {
            args: Args::empty(),
            command_name: String::new(),
            input_pattern: Pattern::new_empty(),
            pattern_copy_buffer: None,
            command_finished: true,
            input: String::new(),
            steps: -1,
            today: DayType::Na,
        }
    }

    pub fn finish(&mut self) {
        self.steps = -1;
        self.command_finished = true;
        self.input_pattern = Pattern::new_empty();
    }
}

#[derive(Clone)]
pub struct Program {
    pub data: ScheduleData,
}

impl Program {
    pub fn new() -> Self {
        Self {
            data: ScheduleData::new(),
        }
    }

    pub fn exit(&self) {
        println!("{}", "|".custom_color(*GREY));
        println!("Exiting...");
        Data::write(self);
        std::process::exit(0);
    }

    pub fn receive(&mut self, pri: &mut ProgramInfo, input: &str) {
        pri.input = input.to_string();
        if !pri.command_finished && (pri.input.to_lowercase() == "stop" || pri.input.to_lowercase() == "cancel") {
            println!("Stopped Command.");
            pri.finish();
            return;
        }

        if !pri.command_finished {
            self.data.receive(pri);
            return;
        }
        let mut parameterless_command: String = input.to_lowercase().clone();
        let mut parameters = String::new();
        let index_at_space = parameterless_command.find(' ');
        if let Some(i) = index_at_space {
            parameters = input[i + 1..input.len()].to_string();
            parameterless_command.replace_range(i..input.len(), "");
        }
        format_command_name(&mut parameterless_command);
        pri.args = Args::get_args(&parameters);
        pri.command_name = parameterless_command.clone();

        self.data.receive(pri);
    }

    pub fn check_patterns(&mut self,hours:i32,mins:i32,dt:DayType){
        let day = self.data.get_day(dt).unwrap();
        day.check_patterns(hours, mins)
    }
}
