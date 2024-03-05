use colored::Colorize;

#[derive(Debug, Clone, PartialEq)]
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
    pub all: Option<bool>,
}

impl Args {
    pub fn empty() -> Self {
        Self {
            name: Ok(String::new()),
            days: Ok(Vec::new()),
            all: None,
        }
    }

    pub fn get_args(string: &str) -> Self {
        let mut res_days: Result<Vec<_>, ArgError> = Err(ArgError::DayFormat);
        let mut res_name: Result<String, ArgError> = Err(ArgError::Empty);
        let mut res_all: Option<bool> = None;

        let mut name: String = String::new();

        let mut parameters = string;

        let unparsed_days: & str;

        let mut anchor: &str = ",";

        if parameters.is_empty() {
            return Self {
                name: res_name,
                days: Err(ArgError::Empty),
                all:res_all,
            };
        }

        let index = parameters.find('[');
        let index2 = parameters.find(']');

        if index.is_some() && index2.is_some() {
            unparsed_days = &parameters[index.unwrap() + 1..index2.unwrap()];

            //println!("{:?}", temp_i);
            if !unparsed_days.contains(anchor) {
                anchor = " ";
            }

            let days:Vec<&str> = unparsed_days.split(anchor.chars().nth(0).unwrap()).collect();

            let mut days_string:Vec<String> = Vec::new();

            for day in &days{
                days_string.push(day.trim().to_owned());
            }

            let mut temp_str:String = parameters.to_string();
            temp_str.replace_range(index.unwrap()..index2.unwrap()+1, "");

            parameters = temp_str.trim();
            //at this point, the whole days array is removed from parameters and
            //what is left is potentially <name> <all>

            let mut end_name = parameters.find(' ');

            if end_name.is_none() {
                end_name = Some(parameters.len());
            } else {
                let all_string = &parameters[end_name.unwrap()..parameters.len()];
                res_all = Some(all_string.contains("all"));
            }

            if !days_string.is_empty() {
                res_days = Ok(days_string);
            } else {
                res_days = Err(ArgError::Empty);
            }

            //name
            name = parameters[0..end_name.unwrap()].trim().to_string();
        } else if index.is_none() && index2.is_none() {
            name = parameters.trim().to_string();
        }

        if name == "all"{
            res_all = Some(true);
            name = "".to_owned();
        }

        if !name.is_empty() {
            res_name = Ok(name);
        }

        let res = Self {
            name: res_name,
            days: res_days,
            all:res_all,
        };

        println!("{res:?}");

        res
    }
}
