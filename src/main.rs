use getopts::Options;
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
    let brief: String = format!("Usage: {} [options]", entered_command);
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

fn list_command() {
    let entries: Vec<Entry> = load_entries();
    if entries.is_empty() {
        println!("No commands registered.");
    } else {
        for (i, cmd) in entries.iter().enumerate() {
            println!(
                "{}. [{}] {} - {}",
                i + 1,
                cmd.category,
                cmd.command,
                cmd.note
            );
        }
    }
}

fn run_command(matches: &getopts::Matches) {
    let index: usize = matches.opt_str("index").unwrap().parse().unwrap_or_else(|e| {
       eprintln!("Failed to parse index: {}", e);
        std::process::exit(1);
    });
    let entries = load_entries();
}

fn main() {
    println!("Hello, world!");
}
