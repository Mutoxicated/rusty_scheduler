#[derive(Debug)]
pub struct Args {
    pub name: String,
    pub days: Vec<String>,
    pub all: bool,
}

impl Args {
    pub fn empty() -> Self {
        Self {
            name: String::new(),
            days: Vec::new(),
            all: false,
        }
    }

    pub fn get_args(string: &String) -> Self {
        let mut days: Vec<String> = Vec::new();
        let mut name: String = String::new();
        let mut all: bool = false;

        let mut str = string.clone();
        let index: Option<usize>;
        let index2: Option<usize>;

        let mut unparsed_days: String;

        let (mut array_starts, mut array_ends) = (true, true);

        let mut anchor: &str = " ";

        if str == "" {
            return Self { name, days, all };
        }

        index = str.find("[");
        index2 = str.find("]");

        if let None = index {
            array_starts = false;
        }
        if let None = index2 {
            array_ends = false;
        }

        if array_starts && array_ends {
            let mut temp_i: Option<usize>;

            unparsed_days = str[index.unwrap() + 1..index2.unwrap()].to_string();

            for _ in 0..7 {
                temp_i = unparsed_days.find(anchor);
                println!("{:?}", temp_i);
                if let None = temp_i {
                    anchor = ",";
                    temp_i = unparsed_days.find(anchor);
                }
                if let None = temp_i {
                    break;
                }
                if 0 == temp_i.unwrap() {
                    continue;
                }

                days.push(unparsed_days[0..temp_i.unwrap() - 1].to_string());

                unparsed_days.replace_range(0..temp_i.unwrap(), "");
                unparsed_days = unparsed_days.trim().to_string();
                println!("Debug: unparsed days-> {}", unparsed_days);
            }

            if unparsed_days != "" {
                days.push(unparsed_days.clone());
            }

            str.replace_range(index.unwrap()..index2.unwrap() + 1, "");

            let mut end_name = str.find(" ");

            if end_name == None {
                end_name = Some(str.len());
            } else {
                let all_string = str[end_name.unwrap()..str.len()].trim();
                all = all_string.contains("all");
            }

            //name
            name = str[0..end_name.unwrap()].trim().to_string();
        } else if !array_starts && !array_ends {
            name = str.trim().to_string();
        }

        let res = Self { name, days, all };

        println!("{res:?}");

        res
    }
}
