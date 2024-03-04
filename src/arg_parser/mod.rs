use colored::Colorize;

#[derive(Debug, PartialEq)]
pub enum ArgError {
    TimeFormat,
    DayFormat,
    InvalidDay,
    Empty,
    InvalidPatternName
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
            ArgError::InvalidPatternName => write!(f, "{} Could not find any pattern with that name!", "Error!".red())
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

        let mut parameters = string;

        let mut unparsed_days: String;

        let (mut array_starts, mut array_ends) = (true, true);

        let mut anchor: &str = " ";

        if parameters.is_empty() {
            return Self {
                name: res_name,
                days: Err(ArgError::Empty),
                all,
            };
        }

        let index = parameters.find('[');
        let index2 = parameters.find(']');

        if index.is_none() {
            array_starts = false;
        }
        if index2.is_none() {
            array_ends = false;
        }

        if array_starts && array_ends {
            let mut temp_i: Option<usize>;

            unparsed_days = parameters[index.unwrap() + 1..index2.unwrap()].to_string();

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
            
            let mut temp_str = parameters.to_string();
            temp_str.replace_range(index.unwrap()..index2.unwrap() + 1, "");

            parameters = temp_str.trim();
            //at this point, the whole days array is removed from parameters and
            //what is left is potentially <name> <all>

            let mut end_name = parameters.find(' ');

            if end_name.is_none() {
                end_name = Some(parameters.len());
            } else {
                let all_string = &parameters[end_name.unwrap()..parameters.len()];
                all = all_string.contains("all");
            }

            if !days.is_empty() {
                res_days = Ok(days);
            } else {
                res_days = Err(ArgError::Empty);
            }

            //name
            name = parameters[0..end_name.unwrap()].trim().to_string();
        } else if !array_starts && !array_ends {
            name = parameters.trim().to_string();
        }

        if name == "all"{
            all = true;
            name = "".to_owned();
        }

        if !name.is_empty() {
            res_name = Ok(name);
        }

        let res = Self {
            name: res_name,
            days: res_days,
            all,
        };

        //println!("{res:?}");

        res
    }
}
