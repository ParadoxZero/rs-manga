mod utilities;
mod commands;

use crate::utilities::commandline_parser;

pub fn process_comandline(args: Vec<String>) {
    let (options, switches, commands) = commandline_parser::parse_input(args);

    
    if cfg!(debug_assertions) {
        println!();
        println!("Input parameters -");
        println!("Options   - {:?}", options);
        println!("Switches  - {:?}", switches);
        println!("Commands  - {:?}", commands);
        println!();
    }

    match commands.first() {
        _ => commands::basic_commands::help()
    }
}
