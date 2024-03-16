use crate::arg_parser::ArgError;
use crate::global::GREY;
use colored::Colorize;

pub fn help() {
    println!("{}", "|".custom_color(*GREY));
    println!("It's important that you understand the vocabulary used in here.\n");

    println!(
        "-You add {} to your schedule, which can be thought of as recurring events.",
        "Patterns".yellow()
    );
    println!("-You can also add {} to your schedule, which are not recurring and \nwill get removed from the schedule once they happen.\n","Special Events".green());

    println!("{} and {} should be self-explanatory.","Patterns".yellow(),"Special Events".green());

    println!("--- COMMANDS ---");
    println!("-today");
    println!("-get_schedule <optional:day(s),alternative:all>");
    println!("-add_pattern [day(s)] <optional:name>");
    println!("-remove_pattern [day(s)] <optional:name> <optional:all>");
    println!("-change_pattern [day(s)] <optional:name> <optional:all>");
    println!("-copy_pattern [day] <name>");
    println!("-paste_pattern [day(s)]");
    //println!("-find_pattern <optional:name> {}","(not yet added)".custom_color(*GREY));
    println!("-clear <optional:days, alternative:all>\n");

    println!("{} add_pattern [Monday,Tuesday]","Example:".bright_blue());
    println!("{} copy_pattern [Sunday] Basketball\n","Example:".bright_blue());

    println!("{} You also can abbreviate the days down to the first three letters! Commands can also be abbreviated.\n","Tip:".green());

    println!("{} add [mon,tue]","Example:".bright_blue());
    println!("{} copy [sun] Basketball\n","Example:".bright_blue());

    println!(
        "{}",
        "(note 1: in the <all> parameter, you either put nothing or the word 'all')".custom_color(*GREY)
    );
    println!("{}","(note 2: in change_pattern, you can put the word 'copy' to change the current copied pattern.)".custom_color(*GREY));

}

pub fn process_time(time: &str) -> (String,String) {
    let mut new_time: String = time.to_string();

    let length: usize = new_time.chars().count();

    let mut index = length;

    for _ in 0..length {
        index -= 1;
        //println!("{}",new_time.chars().nth(index).unwrap());
        if new_time.chars().nth(index-2).unwrap() == ':' {
            //new_time.remove(index);
            return (new_time[0..index-2].to_owned(),new_time[index-1..index+1].to_owned());
        }
        new_time.remove(index);
    }

    (new_time,"0".to_string())
}

pub fn milli_to_nano(num: u32) -> u32 {
    num * 1000000
}

pub fn yes_or_no(string: String) -> bool {
    string.contains('y')
}

fn get_hour_str(time: &str) -> Option<String> {
    let idx = time.find(':');
    idx?;
    if idx.unwrap() + 1 == time.len() {
        return Some("".to_string());
    }
    let hour = time.trim()[0..idx.unwrap()].to_string();
    Some(hour)
}

fn get_minutes_str(time: &str) -> Option<String> {
    let idx = time.find(':');
    idx?;
    if idx.unwrap() + 1 == time.len() {
        return Some("".to_string());
    }
    let minutes = time.trim()[idx.unwrap() + 1..time.len()].to_string();
    Some(minutes)
}

pub fn format_time(time: &str) -> Result<String, ArgError> {
    let mut formatted_time: String = String::from(time);

    if formatted_time.len() == 1 {
        if !formatted_time.chars().nth(0).unwrap().is_ascii_digit(){
            return Err(ArgError::TimeFormat);
        }
        let mut full_str:String = "0".to_string();
        full_str.push_str(formatted_time.as_str());
        formatted_time = full_str;
        //println!("{formatted_time}");
    }

    if formatted_time.len() == 2 {
        let prcsed_str = process_str_num(&formatted_time);
        if prcsed_str.is_none() {
            return Err(ArgError::TimeFormat);
        }
        let res: Result<i32, _> = formatted_time.parse();
        if let Ok(mut num) = res {
            if num > 23 {
                num = 24;
            }
            let mut stringified = num.to_string();
            stringified.push_str(":00");
            return Ok(stringified);
        }
    }

    let hour = process_str_num(&get_hour_str(&formatted_time).unwrap_or("".to_owned()));
    let minutes = process_str_num(&get_minutes_str(&formatted_time).unwrap_or("".to_owned()));

    if let Some(ref str) = hour {
        if str.is_empty() {
            return Err(ArgError::TimeFormat);
        }
    }else {
        return Err(ArgError::TimeFormat);
    }
    if let Some(ref str) = minutes {
        if str.is_empty() {
            return Err(ArgError::TimeFormat);
        }
    }else {
        return Err(ArgError::TimeFormat);
    }

    let minutes: i32 = minutes.unwrap().parse().unwrap();
    if minutes > 59 {
        formatted_time.replace_range(formatted_time.find(':').unwrap()+1..formatted_time.len(), "59");
    }

    Ok(formatted_time)
}

pub fn format_command_name(command: &mut String) {
    if command == "today" || command == "clear" {
        return;
    }
    //println!("{command}");
    if !command.contains('_') {
        if command == "get" {
            command.push_str("_schedule");
        } else {
            command.push_str("_pattern");
        }
    }
}

pub fn limit_to(string:String,limit:usize)->String{
    if string.len() > limit {
        return string[0..limit].to_owned();
    }
    string
}

pub fn process_str_num(mut str:&str) -> Option<String> {
    
    let mut processed_str:String = String::new();
    for char in str.chars().collect::<Vec<char>>() {
        if char.is_ascii_digit() {
            processed_str.push(char);
        }
    }

    if processed_str.is_empty() {
        return None
    }

    Some(processed_str)
}