use std::io::stdin;

pub fn milli_to_nano(num: u32) -> u32 {
    return num * 1000000;
}

pub fn get_input(what_to_say: &str) -> String {
    println!("{what_to_say}");
    let mut copy_receiver: String = String::new();
    stdin().read_line(&mut copy_receiver).unwrap();
    return copy_receiver.trim().to_string();
}

pub fn yes_or_no(string:String) -> bool {
    return string.contains("y");
}