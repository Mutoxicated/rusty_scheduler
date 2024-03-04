mod win;
mod arg_parser;
mod global;
mod program;
mod pst_data;
mod time;
mod utils;

use chrono::{Datelike, Local, Timelike};
use colored::{ColoredString, Colorize};
use global::*;
use program::{Program, ProgramInfo};
use pst_data::Data;
use std::{borrow::BorrowMut, io::stdin, sync::{Arc, Mutex}, thread::{self, sleep}, time::Duration};
use time::{day::Day,Time};
use utils::*;

use tray_item::{TrayItem,IconSource};

fn intro(logo_lines:&[ColoredString],program:&Arc<Mutex<Program>>,program_info:&Arc<Mutex<ProgramInfo>>){    
    // let tray = TrayItem::new(
    //     "Tray Example",
    //     IconSource::Resource(""),
    // )
    // .unwrap();

    // tray.add_menu_item("Open", || {

    // });

    println!("Welcome to the...");

    let mut index: usize = 0;
    sleep(Duration::new(0, milli_to_nano(250)));
    loop {
        sleep(Duration::new(0, milli_to_nano(50)));

        println!("{}", logo_lines[index]);

        index += 1;

        if index == logo_lines.len() {
            body(logo_lines, program, program_info);
            break;
        }
    }
}

fn body(logo_lines:&[ColoredString] ,program:&Arc<Mutex<Program>>,program_info:&Arc<Mutex<ProgramInfo>>) {
    let mut input: String = String::new();
    loop {
        stdin().read_line(&mut input).unwrap();

        if program_info.lock().unwrap().command_finished {
            match input.trim() {
                "min" => {
                    program.as_ref().lock().unwrap().console.min();
                }
                "hide" => {
                    program.as_ref().lock().unwrap().console.hide();
                }
                "help" => {
                    help();
                    input.clear();
                    continue;
                }
                "exit" => {
                    println!("{}", "|".custom_color(*GREY));
                    println!("Exiting...");
                    break;
                },
                "intro" => {
                    intro(logo_lines,program,program_info);
                    continue;
                }
                _ => (),
            };
        }

        program.lock().unwrap().receive(&mut program_info.lock().unwrap(), input.trim());

        input.clear();
    }
}

fn main() {
    let program:Arc<Mutex<Program>> = Arc::new(Mutex::new(Program::new()));
    let program_info:Arc<Mutex<ProgramInfo>> = Arc::new(Mutex::new(ProgramInfo::new()));

    //The idea with this variable is that, since Data implements Drop, when main ends 
    //and the data goes out of scope, it will also call the 'Drop' function from the Drop trait
    //to free the memory, but this also lets us use this as an "OnExit" callback. This wont
    //really work for crashes, or direct exit calls, but it will cover a good amount of situations.

    Data::read(program.as_ref().lock().unwrap().borrow_mut());

    let day_time = Local::now();
    let (time,_) = process_time(&day_time.time().to_string());
    let day = day_time.weekday() as u32;
    program_info.as_ref().lock().unwrap().today = Day::from_u32(day);

    let logo_lines: Arc<Vec<ColoredString>> = Arc::new(vec![
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
    ]);

    println!("Welcome to the...");

    sleep(Duration::new(0, milli_to_nano(250)));
    for i in 0..logo_lines.len() {
        sleep(Duration::new(0, milli_to_nano(50)));

        println!("{}", logo_lines[i]);
    }

    println!("Today is {:?} and the time is {time}.", program_info.as_ref().lock().unwrap().today);
    sleep(Duration::new(0, milli_to_nano(250)));
    println!("Here is uour schedule for today:");
    sleep(Duration::new(0, milli_to_nano(250)));
    println!("{:?}", program_info.as_ref().lock().unwrap().today);
    program.as_ref().lock().unwrap().data
        .get_day(program_info.as_ref().lock().unwrap().today.clone())
        .unwrap()
        .present_patterns(true);

    println!("{}help{}","Type '".green(),"' if you're unfamiliar with the commands.".green());

    let cloned_program = Arc::clone(&program);
    let cloned_program2 = Arc::clone(&program);
    let cloned_program3 = Arc::clone(&program);
    let cloned_program4 = Arc::clone(&program);
    let cloned_program_info = Arc::clone(&program_info);

    //Setup System Tray
    let mut tray = TrayItem::new("Rusty Scheduler", IconSource::Resource("rusty_sched")).unwrap();

    tray.add_menu_item("Show", move || {
        cloned_program2.as_ref().lock().unwrap().console.show();
    })
    .unwrap();

    tray.add_menu_item("Hide", move || {
        cloned_program3.as_ref().lock().unwrap().console.hide();
    })
    .unwrap();

    tray.add_menu_item("Exit", move || {
        cloned_program4.as_ref().lock().unwrap().exit();
    })
    .unwrap();

    thread::spawn(move || {
        let day_time = Local::now();
        let (time,secs) = process_time(&day_time.time().to_string());
        let (hours,mins,seconds) = 
            (get_hour(time.as_str()),
            get_minutes(time.as_str()),
            secs.parse().unwrap());
        
        let mut timee:Time = Time::new(
            hours,
            mins,
            seconds);

        let mut next_minute:u64 = 1;
        loop {
            sleep(Duration::new(next_minute,0));
            let now = Local::now();
            timee.update(now.hour(),now.minute(),now.second());
            next_minute = 60-timee.seconds;
            //println!("{next_minute}");
            //timee.print();
            cloned_program.lock().unwrap().check_patterns(timee.hours,timee.minutes,cloned_program_info.lock().unwrap().today.clone());
        }
    });

    body(&logo_lines,&program,&program_info);

    Data::write(program.as_ref().lock().unwrap().borrow_mut());
}