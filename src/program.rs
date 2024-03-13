use chrono::NaiveDateTime;
use windows::core::PCSTR;

use crate::arg_parser::Args;
use crate::pst_data::Data;
use crate::time::{day::DayType,Pattern,day::PatternDetectionType,PatternInfo,ScheduleData};
use crate::utils::format_command_name;
use crate::win::ConsoleWindow;

#[derive(Default)]
pub struct CommandInfoBuffer {
    pub input_pattern: PatternInfo,
    pub all:Option<PatternDetectionType>,
    pub valid_daytypes:Option<Vec<DayType>>,
    pub index:Option<usize>,
}


pub trait Receive {
    fn receive(&mut self, pr: &mut ProgramInfo);
}

pub struct ProgramInfo {
    pub command_name: String,
    pub args: Args,
    pub cmib: CommandInfoBuffer,
    pub pattern_copy_buffer: Option<Pattern>,
    pub command_finished: bool,
    pub input: String,
    pub asked: bool,
    pub today: DayType,
}

impl ProgramInfo {
    pub fn new() -> Self {
        Self {
            args: Args::empty(),
            command_name: String::new(),
            cmib: CommandInfoBuffer::default(),
            pattern_copy_buffer: None,
            command_finished: true,
            input: String::new(),
            asked: false,
            today: DayType::Na,
        }
    }

    pub fn finish(&mut self) {
        self.asked = false;
        self.command_finished = true;
        self.cmib = CommandInfoBuffer::default();
    }
}

#[derive(Clone)]
pub struct Program {
    pub data: ScheduleData,
    pub console: ConsoleWindow
}

impl Program {
    pub fn new() -> Self {
        Self {
            data: ScheduleData::new(),
            console: ConsoleWindow::init(PCSTR::from_raw("Rusty_Scheduler\0".as_bytes().as_ptr()))
        }
    }
    
    fn command_is_stop(command_name:String)->bool{
        command_name == "stop" || command_name == "cancel"
    }

    pub fn receive(&mut self, pri: &mut ProgramInfo, input: String) {
        pri.input = input.to_string();
        //println!("Debug: command_finished->{}",pri.command_finished);

        if !pri.command_finished && Program::command_is_stop(pri.input.to_lowercase()) {
            println!("Stopped Command.");
            pri.finish();
            return;
        }

        if !pri.command_finished {
            self.data.receive(pri);
            return;
        }

        let mut parameterless_command: String = input.to_lowercase();
        let mut parameters:String = "".to_owned();

        let index_at_space = parameterless_command.find(' ');

        if let Some(i) = index_at_space {
            parameters = input[i + 1..input.len()].to_owned();
            parameterless_command = input[0..i].to_string();
        }
        
        format_command_name(&mut parameterless_command);

        pri.args = Args::get_args(parameters.as_str());
        pri.command_name = parameterless_command;
        pri.command_finished = false;

        self.data.receive(pri);
    }

    pub fn check_patterns(&mut self,time:NaiveDateTime,dt:DayType){
        let day = self.data.get_day(dt).unwrap();
        day.check_patterns(time)
    }

    pub fn exit(&self){
        Data::write(self);
        std::process::exit(0);
    }
}
