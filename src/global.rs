use colored::CustomColor;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref DAY_SPACE: String = String::from("           ");
    pub static ref GREY: CustomColor = CustomColor::new(100, 100, 100);
    pub static ref RUSTY: CustomColor = CustomColor::new(200, 60, 0);
    pub static ref LIGHTBLUE: CustomColor = CustomColor::new(30, 150, 255);
}
