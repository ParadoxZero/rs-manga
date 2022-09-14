mod commands;
mod utilities;

use commands::basic_commands;
use utilities::commandline_parser;

pub fn process_comandline(args: Vec<String>) {
    let (options, switches, commands) = commandline_parser::parse_input(args);

    dbg_print_params(options, switches, &commands);

    if let Some(command) = commands.first().to_owned() {
        handle_command(command);
    } else {
        basic_commands::help();
    }
}

fn handle_command(command: &String) {
    let is_cmd = |y: &str| -> bool {
        return command.eq_ignore_ascii_case(y);
    };

    if is_cmd("help") {
        basic_commands::help();
    } else if is_cmd("search") {
    } else if is_cmd("latest") {
    } else {
        // Assume rest of the words are manga title
        println!("Finding details of manga - \"{command}\" ");
    }
}

fn dbg_print_params(
    options: std::collections::HashMap<String, String>,
    switches: Vec<String>,
    commands: &Vec<String>,
) {
    if cfg!(debug_assertions) {
        println!();
        println!("[Debug Info] Input parameters -");
        println!("[Debug Info] Options   - {:?}", options);
        println!("[Debug Info] Switches  - {:?}", switches);
        println!("[Debug Info] Commands  - {:?}", commands);
        println!();
    }
}
