pub fn milli_to_nano(num: u32) -> u32 {
    return num * 1000000;
}

pub fn assign_str_from_input(what_to_say: &str) -> String {
    use std::io::stdin;
    println!("{what_to_say}");
    let mut copy_receiver: String = String::new();
    stdin().read_line(&mut copy_receiver).unwrap();
    return copy_receiver.trim().to_string();
}

pub fn yes_or_no(what_to_say: &str) -> bool {
    use std::io::stdin;
    println!("{what_to_say}");
    let mut copy_receiver: String = String::new();
    stdin().read_line(&mut copy_receiver).unwrap();
    return copy_receiver.contains("y");
}

pub fn parse_arguments(string: &String) -> Vec<String> {
    let mut str = string.clone();
    let mut args: Vec<String> = Vec::new();
    let mut previous_index = 0;
    let mut index: Option<usize>;

    if str == "" {
        return args;
    }

    for _ in 0..3 {
        index = str.find(" ");
        if let None = index {
            break;
        }
        if previous_index == index.unwrap() {
            continue;
        }

        args.push(str[previous_index..index.unwrap()].to_string());

        str.replace_range(previous_index..index.unwrap() + 1, "");

        previous_index = index.unwrap();
    }

    if str != "" {
        args.push(str.clone());
    }

    args
}
