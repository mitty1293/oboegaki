mod commands;
mod data;

use getopts::Options;
use std::env;

fn print_help(entered_command: &str, opts: Options) {
    let brief: String = format!(
        "Usage: {} <subcommand> [options]\n\nSubcommands:\n\
        \tadd    Add a new command\n\
        \tlist   List all registered commands\n\
        \trun    Run a command by its index\n\
        \tdelete Delete a command by its index\n",
        entered_command
    );
    print!("{}", opts.usage(&brief));
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let primary_command = args[0].clone();

    let mut opts = Options::new();
    opts.optopt("", "command", "command to add", "COMMAND");
    opts.optopt("", "category", "category of the command", "CATEGORY");
    opts.optopt("", "note", "note for the command", "NOTE");
    opts.optopt("", "index", "index of the command to run or delete", "INDEX");
    opts.optflag("h", "help", "print this help menu");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => {
            eprintln!("Error: {}", f);
            print_help(&primary_command, opts);
            return;
        }
    };

    if matches.opt_present("h") {
        print_help(&primary_command, opts);
        return;
    }

    if let Some(subcommand) = args.get(1) {
        match subcommand.as_str() {
            "add" => commands::add_command(&matches),
            "list" => commands::list_commands(),
            "run" => commands::run_command(&matches),
            "delete" => commands::delete_command(&matches),
            _ => {
                eprintln!("Unknown command: {}", subcommand);
                print_help(&primary_command, opts);
            }
        }
    } else {
        print_help(&primary_command, opts);
    }
}
