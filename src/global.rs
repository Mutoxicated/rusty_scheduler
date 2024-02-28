use colored::CustomColor;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref SMALL_IDENT: String = String::from("    ");
    pub static ref IDENT: String = String::from("    ");
    pub static ref GREY: CustomColor = CustomColor::new(100, 100, 100);
}
