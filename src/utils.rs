pub fn milli_to_nano(num: u32) -> u32 {
    return num * 1000000;
}

pub fn yes_or_no(string: String) -> bool {
    return string.contains("y");
}
