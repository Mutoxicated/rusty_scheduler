use crate::pst_data::Data;
use crate::time::day::DayType;
use crate::time::ScheduleData;
use crate::time::Pattern;
use crate::arg_parser::Args;

pub trait Receive {
    fn receive<'a>(&mut self, pr:&mut ProgramInfo);
}

pub struct ProgramInfo {
    pub command_name:String,
    pub args:Args,
    pub input_pattern:Pattern,
    pub command_finished:bool,
    pub input:String,
    pub steps:i32
}

impl ProgramInfo {
    pub fn new() -> Self {
        Self {
            args:Args::empty(),
            command_name:String::new(),
            input_pattern:Pattern::new_empty(),
            command_finished:true,
            input:String::new(),
            steps:-1,
        }
    }

    pub fn finish(&mut self){
        self.steps = -1;
    }
}

#[derive(Clone)]
pub struct Program {
    pub data: ScheduleData,
    pub today: DayType
}

impl Program {
    pub fn new() -> Self {
        Self {
            data: ScheduleData::new(),
            today: DayType::Na
        }
    }

    pub fn exit(&self) {
        println!("|");
        println!("Exiting...");
        Data::write(&self);
        std::process::exit(0);
    }

    pub fn receive<'a>(&mut self, pri:&mut ProgramInfo, input:&String){
        pri.input = input.clone();
        if pri.input.to_lowercase() == "stop" || pri.input.to_lowercase() == "cancel" {
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
        let index_at_space = parameterless_command.find(" ");
        if let Some(i) = index_at_space {
            parameters = input[i + 1..input.len()].to_string();
            parameterless_command.replace_range(i..input.len(), "");
        }
        pri.args = Args::get_args(&parameters);
        pri.command_name = parameterless_command.clone();

        self.data.receive(pri);
    }
}
