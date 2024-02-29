mod arg_parser;
mod global;
mod program;
mod pst_data;
mod time;
mod utils;

use chrono::{Datelike, Local};
use colored::{ColoredString, Colorize};
use global::*;
use program::{Program, ProgramInfo};
use pst_data::Data;
use std::{io::stdin, thread::sleep, time::Duration};
use time::day::Day;
use utils::{help, milli_to_nano, process_time};

fn main() {
    let mut pr = Program::new();
    let mut pri = ProgramInfo::new();

    //pr.data.receive("-remove_pattern Monday Goobo".to_string());
    Data::read(&mut pr);

    let mut input: String = String::new();
    let logo_lines: Vec<ColoredString> = vec![
        r"   ____              __       ".custom_color(*RUSTY),
        r"   / __ \__  _______/ /___  __".custom_color(*RUSTY),
        r"  / /_/ / / / / ___/ __/ / / /".custom_color(*RUSTY),
        r" / _, _/ /_/ (__  ) /_/ /_/ / ".custom_color(*RUSTY),
        r"/_/ |_|\__,_/____/\__/\__, /  ".custom_color(*RUSTY),
        r"                     /____/   ".custom_color(*RUSTY),
        r"   _____      __             __      __         ".custom_color(*LIGHTBLUE),
        r"  / ___/_____/ /_  ___  ____/ /_  __/ /__  _____".custom_color(*LIGHTBLUE),
        r"  \__ \/ ___/ __ \/ _ \/ __  / / / / / _ \/ ___/".custom_color(*LIGHTBLUE),
        r" ___/ / /__/ / / /  __/ /_/ / /_/ / /  __/ /    ".custom_color(*LIGHTBLUE),
        r"/____/\___/_/ /_/\___/\__,_/\__,_/_/\___/_/     ".custom_color(*LIGHTBLUE),
        r"                                                ".custom_color(*LIGHTBLUE),
    ];

    let day_time = Local::now();
    let time = process_time(day_time.time().to_string());
    let day = day_time.weekday() as u32;
    pri.today = Day::from_u32(day);

    println!("Welcome to the...");

    let mut index: usize = 0;
    sleep(Duration::new(0, milli_to_nano(250)));
    loop {
        sleep(Duration::new(0, milli_to_nano(50)));

        println!("{}", logo_lines[index]);

        index += 1;

        if index == logo_lines.len() {
            break;
        }
    }

    println!("Today is {:?} and the time is {time}.", pri.today);
    sleep(Duration::new(0, milli_to_nano(250)));
    println!("Press enter to take a look of your schedule for today:");
    stdin().read_line(&mut input).unwrap();

    println!("{:?}", pri.today);
    pr.data
        .get_day(pri.today.clone())
        .unwrap()
        .present_patterns(true);

    println!("Type 'help' if you're unfamiliar with the commands.");

    let mut input: String = String::new();

    loop {
        stdin().read_line(&mut input).unwrap();

        match input.trim() {
            "help" => {
                help();
                input.clear();
                continue;
            }
            "exit" => {
                pr.exit();
                input.clear();
                continue;
            }
            _ => (),
        };

        pr.receive(&mut pri, &input.trim().to_string());

        input.clear();
    }
}
