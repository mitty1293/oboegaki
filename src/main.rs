use getopts::Options;
use std::env;
use std::fs::{File, OpenOptions};
use std::io::{BufReader, BufWriter};
use serde::{Deserialize, Serialize};

const DATA_FILE: &str = "oboegaki.json";

#[derive(Serialize, Deserialize, Debug)]
struct Entry {
    command: String,
    category: String,
    note: String,
}

fn load_entries() -> Vec<Entry> {
    match File::open(DATA_FILE) {
        Ok(file) => {
            let reader: BufReader<File> = BufReader::new(file);
            let entries: Vec<Entry> = serde_json::from_reader(reader).unwrap_or_else(|e| {
                eprintln!("Failed to parse json: {}", e);
                vec![]
            });
            entries
        }
        Err(e) => {
            if e.kind() == std::io::ErrorKind::NotFound {
                // ファイルが存在しない場合は空のベクタを返す
                vec![]
            } else {
                // 他のエラーの場合はエラーメッセージを表示して終了
                eprintln!("Failed to open file: {}", e);
                std::process::exit(1);
            }
        }
    }
}

fn save_entries(entries: &Vec<Entry>) {
    let file: File = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(DATA_FILE)
        .unwrap_or_else(|e| {
            eprintln!("Failed to open file for writing: {}", e);
            std::process::exit(1);
        });
    let writer = BufWriter::new(file);
    serde_json::to_writer_pretty(writer, entries).unwrap_or_else(|e| {
        eprintln!("Failed to write JSON: {}", e);
    });
}

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

fn add_command(matches: &getopts::Matches) {
    let command = matches.opt_str("command").unwrap();
    let category = matches.opt_str("category").unwrap();
    let note = matches.opt_str("note").unwrap();

    let mut entries: Vec<Entry> = load_entries();
    entries.push(Entry{
        command,
        category,
        note,
    });
    save_entries(&entries);
    println!("Command added.");
}

fn list_commands() {
    let entries: Vec<Entry> = load_entries();
    if entries.is_empty() {
        println!("No commands registered.");
    } else {
        println!("{:<5} {:<10} {:<30} {:<4}", "Index", "Category", "Command", "Note");
        println!("{:-<5} {:-<10} {:-<30} {:-<4}", "", "", "", "");
        for (i, entry) in entries.iter().enumerate() {
            println!(
                "{:<5} {:<10} {:<30} {}",
                i + 1,
                entry.category,
                entry.command,
                entry.note
            );
        }
    }
}

fn parse_index(matches: &getopts::Matches) -> Result<usize, String> {
    let index = matches.opt_str("index")
        .ok_or_else(|| "Index not provided".to_string())?
        .parse::<usize>()
        .map_err(|e| format!("Failed to parse index: {}", e));
    index
}

fn run_command(matches: &getopts::Matches) {
    let index = match parse_index(matches) {
        Ok(i) => i,
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    };
    let entries = load_entries();
    if let Some(entry) = entries.get(index - 1) {
        println!("Running: {}", entry.command);
        let command_segments: Vec<&str> = entry.command.split_whitespace().collect();
        if let Some((primary_command, command_options)) = command_segments.split_first() {
            let _ = std::process::Command::new(primary_command).args(command_options).status();
        }
    } else {
        println!("Invalid command index.");
    }
}

fn delete_command(matches: &getopts::Matches) {
    let index = match parse_index(matches) {
        Ok(i) => i,
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    };
    let mut entries = load_entries();
    if index == 0 || entries.len() < index {
        println!("Invalid command index.");
    } else {
        let delete_entry = entries.remove(index - 1);
        save_entries(&entries);
        println!(
            "Command deleted: {:<5} {:<10} {:<30} {}",
            index,
            delete_entry.category,
            delete_entry.command,
            delete_entry.note
        );
    }
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
            "add" => add_command(&matches),
            "list" => list_commands(),
            "run" => run_command(&matches),
            "delete" => delete_command(&matches),
            _ => {
                eprintln!("Unknown command: {}", subcommand);
                print_help(&primary_command, opts);
            }
        }
    } else {
        print_help(&primary_command, opts);
    }
}
