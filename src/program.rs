use crate::pst_data::Data;
use crate::time::day::DayType;
use crate::time::ScheduleData;

pub trait Receive {
    fn receive(&mut self, input: String);
}

pub struct Program {
    pub data: ScheduleData,
    pub today: DayType,
}

impl Program {
    pub fn new() -> Self {
        Self {
            data: ScheduleData::new(),
            today: DayType::Na,
        }
    }

    pub fn exit(&self) {
        println!("Exiting...");
        Data::write(&self);
        std::process::exit(0);
    }
}

impl Receive for Program {
    fn receive(&mut self, input: String) {
        self.data.receive(input);
    }
}
