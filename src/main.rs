mod arg_parser;
mod global;
mod program;
mod pst_data;
mod time;
mod utils;

use chrono::{Datelike, Local};
use pst_data::Data;
use time::day::Day;
use utils::milli_to_nano;

fn process_time(time: String) -> String {
    let mut new_time: String = time;

    let length: usize = new_time.chars().count();

    let mut index = length;

    for _ in 0..length {
        index = index - 1;
        //println!("{}",new_time.chars().nth(index).unwrap());
        if new_time.chars().nth(index).unwrap() == ':' {
            new_time.remove(index);
            return new_time;
        }
        new_time.remove(index);
    }

    new_time
}

// fn pow(x:f32,y:usize)->f32{
//     let mut result = x;
//     for i in 0..y-1{
//         result = result * result;
//     }
//     result
// }

// fn f(x:f32){
//     println!("{}",(2.0*pow(x+2.0,3))-(17.0*pow(x+2.0,2))+(42.0*(x+2.0))-29.0);
// }

use colored::Colorize;
use std::io::stdin;
use std::thread::sleep;
use std::time::Duration;

fn help() {
    println!("|");
    println!("It's important that you understand the vocabulary used in here.");
    println!("");
    println!(
        "-You add {} to your schedule, which can be thought of as recurring events.",
        "Patterns".yellow()
    );
    println!("-You can also add {} to your schedule, which are not recurring and \nwill get removed from the schedule once they happen.","Special Events".green());
    println!("");
    println!("--- COMMANDS ---");
    println!("-get_schedule [optional:day(s)]");
    println!("-add_pattern [day(s)]");
    println!("-remove_pattern [day(s)] <optional:name> <optional:all>");
    println!("-change_pattern [day(s)] <optional:name>");
    println!("-clear_patterns [day(s)]");
    println!("-copy_pattern [day] <optional:name>");
    println!("-paste_pattern [day(s)]");
    println!("-find_pattern <optional:name>");
    println!("");
    println!("Example: add_pattern [Monday,Tuesday]");
    println!("Example: copy_pattern [Sunday] Basketball");
    println!("");
    println!("{}","(note: you can put 'today' in [day])".custom_color(*GREY));
    println!("{}","(note: in the <all> parameter, you either put nothing or the word 'all')".custom_color(*GREY));
}

use program::{Program,ProgramInfo};

use crate::global::GREY;

fn main() {
    let mut pr = Program::new();
    let mut pri = ProgramInfo::new();

    //pr.data.receive("-remove_pattern Monday Goobo".to_string());
    Data::read(&mut pr);

    let mut input: String = String::new();
    let logo_lines: Vec<&str> = vec![
        r"   ____              __       ",
        r"   / __ \__  _______/ /___  __",
        r"  / /_/ / / / / ___/ __/ / / /",
        r" / _, _/ /_/ (__  ) /_/ /_/ / ",
        r"/_/ |_|\__,_/____/\__/\__, /  ",
        r"                     /____/   ",
        r"   _____      __             __      __         ",
        r"  / ___/_____/ /_  ___  ____/ /_  __/ /__  _____",
        r"  \__ \/ ___/ __ \/ _ \/ __  / / / / / _ \/ ___/",
        r" ___/ / /__/ / / /  __/ /_/ / /_/ / /  __/ /    ",
        r"/____/\___/_/ /_/\___/\__,_/\__,_/_/\___/_/     ",
        r"                                                  ",
    ];

    let day_time = Local::now();
    let time = process_time(day_time.time().to_string());
    let day = day_time.weekday() as u32;
    let day_type = Day::from_u32(day);
    pr.today = day_type.clone();

    println!("Welcome to the...");

    let mut index: usize = 0;
    sleep(Duration::new(0, milli_to_nano(250)));
    loop {
        sleep(Duration::new(0, milli_to_nano(50)));

        println!("{}", logo_lines[index]);

        index = index + 1;

        if index == logo_lines.len() {
            break;
        }
    }

    println!("Today is {day_type:?} and the time is {time}.");
    sleep(Duration::new(0, milli_to_nano(250)));
    println!("Press enter to take a look of your schedule for today:");
    stdin().read_line(&mut input).unwrap();

    println!("{day_type:?}");
    pr.data.get_day(day_type).unwrap().present_patterns();
    
    println!("Type 'help' if you're unfamiliar with the commands.");

    let mut input: String = String::new();

    loop {
        stdin().read_line(&mut input).unwrap();

        match input.trim() {
            "help" => help(),
            "exit" => pr.exit(),
            _ => (),
        };

        pr.receive(&mut pri,&input.trim().to_string());

        input.clear();
    }
}
