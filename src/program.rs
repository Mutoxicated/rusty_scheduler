use crate::pst_data::ScheduleData;
use crate::pst_data::Data;


pub struct Program {
    pub data:ScheduleData
}

impl Program{
    pub fn new() -> Self{
        Self{
            data:ScheduleData::new()
        }
    }

    pub fn exit(&self){
        Data::write(&self);
        std::process::exit(0);
    }
}