use colored::Colorize;

#[derive(Debug, PartialEq)]
pub enum ArgError {
    TimeFormat,
    DayFormat,
    InvalidDay,
    Empty,
}

impl std::fmt::Display for ArgError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ArgError::DayFormat => write!(
                f,
                "{} You must've forgotten to start or end the array of the days (using [])!",
                "Error!".red()
            ),
            ArgError::TimeFormat => write!(
                f,
                "{} Time must be given in a 24-hour format like so: {}",
                "Error!".red(),
                "XX-XX".blue()
            ),
            ArgError::Empty => write!(f, "{} You didn't pass anything!", "Error!".red()),
            ArgError::InvalidDay => write!(f, "{} No valid days were given!", "Error!".red()),
        }
    }
}

#[derive(Debug)]
pub struct Args {
    pub name: Result<String, ArgError>,
    pub days: Result<Vec<String>, ArgError>,
    pub all: bool,
}

impl Args {
    pub fn empty() -> Self {
        Self {
            name: Ok(String::new()),
            days: Ok(Vec::new()),
            all: false,
        }
    }

    pub fn get_args(string: &str) -> Self {
        let mut res_days: Result<Vec<_>, ArgError> = Err(ArgError::DayFormat);
        let mut res_name: Result<String, ArgError> = Err(ArgError::Empty);

        let mut days: Vec<String> = Vec::new();
        let mut name: String = String::new();
        let mut all: bool = false;

        let mut str = string;

        let mut unparsed_days: String;

        let (mut array_starts, mut array_ends) = (true, true);

        let mut anchor: &str = " ";

        if str.is_empty() {
            return Self {
                name: res_name,
                days: Err(ArgError::Empty),
                all,
            };
        }

        let index = str.find('[');
        let index2 = str.find(']');

        if index.is_none() {
            array_starts = false;
        }
        if index2.is_none() {
            array_ends = false;
        }

        if array_starts && array_ends {
            let mut temp_i: Option<usize>;

            unparsed_days = str[index.unwrap() + 1..index2.unwrap()].to_string();

            let mut valid: bool = true;
            if unparsed_days.find('[').is_some() {
                valid = false;
            }
            if unparsed_days.find(']').is_some() {
                valid = false;
            }
            if valid {
                for _ in 0..7 {
                    temp_i = unparsed_days.find(anchor);
                    //println!("{:?}", temp_i);
                    if temp_i.is_none() {
                        anchor = ",";
                        temp_i = unparsed_days.find(anchor);
                    }
                    if temp_i.is_none() {
                        break;
                    }
                    if 0 == temp_i.unwrap() {
                        continue;
                    }
                    if unparsed_days[0..temp_i.unwrap() - 1] != *"" {
                        days.push(unparsed_days[0..temp_i.unwrap()].trim().to_string());
                    }

                    unparsed_days.replace_range(0..temp_i.unwrap() + 1, "");
                    unparsed_days = unparsed_days.trim().to_string();
                    //println!("Debug: unparsed days-> {}", unparsed_days);
                }

                if !unparsed_days.is_empty() {
                    days.push(unparsed_days.clone());
                }
            }

            str.to_string().replace_range(index.unwrap()..index2.unwrap() + 1, "");

            str = str.trim();

            let mut end_name = str.find(' ');

            //println!("Args: end_name->{end_name:?}");

            if end_name.is_none() {
                end_name = Some(str.len());
            } else {
                let all_string = &str[end_name.unwrap()..str.len()];
                all = all_string.contains("all");
            }

            if !days.is_empty() {
                res_days = Ok(days);
            } else {
                res_days = Err(ArgError::Empty);
            }

            //name
            name = str[0..end_name.unwrap()].trim().to_string();
        } else if !array_starts && !array_ends {
            name = str.trim().to_string();
        }

        if !name.is_empty() {
            res_name = Ok(name);
        }

        let res = Self {
            name: res_name,
            days: res_days,
            all,
        };

        println!("{res:?}");

        res
    }
}
