use std::env;
use std::fs::{File, OpenOptions};
use std::io::{BufReader, BufWriter};
use std::path::PathBuf;
use serde::{Deserialize, Serialize};

fn get_data_file_path() -> PathBuf {
    let config_dir = env::var("XDG_CONFIG_HOME").unwrap_or_else(|_| {
        let home_dir = env::var("HOME").expect("Failed to get home directory");
        format!("{}/.config", home_dir)
    });
    let data_dir = format!("{}/oboegaki", config_dir);
    std::fs::create_dir_all(&data_dir).expect("Failed to create config directory");
    PathBuf::from(format!("{}/commands.json", data_dir))
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Entry {
    pub command: String,
    pub category: String,
    pub note: String,
}

pub fn load_entries() -> Vec<Entry> {
    let data_file_path = get_data_file_path();
    match File::open(&data_file_path) {
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

pub fn save_entries(entries: &Vec<Entry>) {
    let data_file_path = get_data_file_path();
    let file: File = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(&data_file_path)
        .unwrap_or_else(|e| {
            eprintln!("Failed to open file for writing: {}", e);
            std::process::exit(1);
        });
    let writer = BufWriter::new(file);
    serde_json::to_writer_pretty(writer, entries).unwrap_or_else(|e| {
        eprintln!("Failed to write JSON: {}", e);
    });
}
