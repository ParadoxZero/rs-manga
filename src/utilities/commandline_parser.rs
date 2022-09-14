use std::{collections::HashMap, str::FromStr};

pub fn parse_input(args: Vec<String>) -> (HashMap<String, String>, Vec<String>, Vec<String>) {
    let mut options: HashMap<String, String> = HashMap::new();
    let mut switches: Vec<String> = Vec::new();
    let mut commands: Vec<String> = Vec::new();
    for arg in args {
        if arg.starts_with("--") {
            let values = arg.replace("--", "");
            let values: (String, String) = match values.split_once('=') {
                None => (values, String::new()),
                Some(a) => (
                    FromStr::from_str(a.0.clone()).expect("Failed to convert to string"),
                    FromStr::from_str(a.1.clone()).expect("Failed to convert to string"),
                ),
            };
            options.insert(values.0, values.1);
        } else if arg.starts_with("-") {
            switches.push(arg.replace("-", ""));
        } else {
            commands.push(arg);
        }
    }
    (options, switches, commands)
}
