use crate::data::{Entry, load_entries, save_entries};

pub fn add_command(matches: &getopts::Matches) {
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

pub fn list_commands() {
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

pub fn run_command(matches: &getopts::Matches) {
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

pub fn delete_command(matches: &getopts::Matches) {
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