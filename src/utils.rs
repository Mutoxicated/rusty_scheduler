use crate::arg_parser::ArgError;

pub fn milli_to_nano(num: u32) -> u32 {
    return num * 1000000;
}

pub fn yes_or_no(string: String) -> bool {
    return string.contains("y");
}

pub fn get_hour(time:&str) -> i32{
    let idx = time.find(":");
    let hour = time
        .trim()[0..idx
            .expect("No number was found when getting the hour of the given time string.")]
        .parse()
        .expect("Couldn't parse the hour of the given time.");
    hour
}

pub fn get_minutes(time:&str) -> i32 {
    let idx = time.find(":");
    let minutes = time
        .trim()[idx.expect("No number was found when getting the hour of the given time string.")+1..time.len()]
        .parse()
        .expect("Couldn't parse the hour of the given time.");
    minutes
}

fn get_hour_str(time:&str) -> Option<String>{
    let idx = time.find(":");
    if let None = idx {
        return None;
    }
    if idx.unwrap()+1 == time.len(){
        return Some("".to_string());
    }
    let hour = time
        .trim()[0..idx.unwrap()]
        .to_string();
    Some(hour)
}

fn get_minutes_str(time:&str) -> Option<String> {
    let idx = time.find(":");
    if let None = idx {
        return None;
    }
    if idx.unwrap()+1 == time.len(){
        return Some("".to_string());
    }
    let minutes = time
        .trim()[idx.unwrap()+1..time.len()]
        .to_string();
    Some(minutes)
}


pub fn format_time(time:&str) -> Result<String,ArgError>{
    let mut formatted_time:String = String::from(time);
    
    let hour = get_hour_str(time);
    let minutes = get_minutes_str(time);

    if time.len() == 2 {
        let res:Result<i32, _> = time.parse();
        if let Ok(mut num) = res {
            if num > 23 {
                num = 24;
            }
            let mut stringified = num.to_string();
            stringified.push_str(":00");
            return Ok(stringified);
        }
    }

    if let None = hour {
        return Err(ArgError::TimeFormat);
    }
    if let Some(ref str) = hour {
        if str == ""{
            return Err(ArgError::TimeFormat);
        }
    }
    if let Some(ref str) = minutes {
        if str == ""{
            return Err(ArgError::TimeFormat);
        }
    }

    let hour_res:Result<String,_> = hour.unwrap().parse();

    if let Err(_) = hour_res {
        return Err(ArgError::TimeFormat);
    }
    let minutes_res:Result<String,_> = minutes.unwrap().parse();
    if let Err(_) = minutes_res {
        return Err(ArgError::TimeFormat);
    }

    if minutes_res.unwrap().len() == 1 {
        formatted_time.insert_str(3,"0");
        return Ok(formatted_time);
    }

    return Ok(formatted_time);
}