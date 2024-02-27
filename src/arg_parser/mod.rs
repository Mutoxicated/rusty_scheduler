struct Args {
    //TO-DO: Use this in the schedule module
    name: String,
    unparsed_days: Vec<String>,
}

impl Args {
    pub fn get_args(string: &String) -> Self {
        let mut days: Vec<String> = Vec::new();
        let mut name: String = String::new();

        let str = string.clone();
        let mut index: Option<usize> = Some(0usize);
        let mut index2: Option<usize> = Some(0usize);

        let mut unparsed_days: String = String::new();

        let (mut array_starts, mut array_ends) = (true, true);

        if str == "" {
            return Self {
                name,
                unparsed_days: days,
            };
        }

        index = str.find("[");
        index2 = str.find("]");

        if let None = index {
            array_starts = true;
        }
        if let None = index2 {
            array_ends = true;
        }

        if array_starts && array_ends {
            let mut temp_i: Option<usize> = Some(0usize);
            let mut temp_i2 = temp_i.clone();
            unparsed_days = str[index.unwrap() + 1..index2.unwrap() - 1].to_string();

            for _ in 0..7 {
                temp_i = unparsed_days.find(" ");
                if let None = temp_i {
                    break;
                }
                if temp_i2.unwrap() == temp_i.unwrap() {
                    continue;
                }

                days.push(unparsed_days[temp_i2.unwrap()..temp_i.unwrap()].to_string());

                unparsed_days.replace_range(temp_i2.unwrap()..temp_i.unwrap() + 1, "");

                temp_i2 = temp_i;
            }

            if unparsed_days != "" {
                days.push(unparsed_days.clone());
            }

            //name
            name = str[index2.unwrap() + 1..str.len()].trim().to_string();
        } else if !array_starts && !array_ends {
            name = str.trim().to_string();
        }

        Self {
            name,
            unparsed_days: days,
        }
    }
}
