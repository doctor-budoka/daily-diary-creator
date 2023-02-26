use std::fs::{File, OpenOptions, create_dir_all,read_to_string};
use std::io::Write;
use std::path::Path;
use std::env::var;
use num_traits::cast::FromPrimitive;
use chrono::prelude::{DateTime, Utc, Datelike, Month};
use ordinal::Ordinal;
use std::process::Command;

const CFG_PATH: &str = "~/.diary_cfg";
const EXTRAS_PATH: &str = "~/.diary_extras";

fn main() {
    let now: DateTime<Utc> = Utc::now();
    let iso_date = now.format("%Y-%m-%d").to_string();
    let file_name = iso_date.to_owned() + ".md";

    let base_folder = get_base_folder();
    let target_dir = base_folder.to_owned() + &now.year().to_string() + "-Diary/";
    let target_path = target_dir.to_string() + &file_name.to_string();

    if !Path::new(&target_dir).exists() {
        println!("Target directory, '{}', doesn't exist. Creating it now!", target_dir);
        create_dir_all(target_dir).expect("Should be able to create directory!");
    }

    if Path::new(&target_path).exists() {
        println!("Diary for today ('{}') already exists", iso_date);
    }
    else {
        let file = OpenOptions::new().create_new(true).write(true).open(target_path);
        match file {
            Ok(file_pointer) => write_header_to_file(file_pointer, now),
            Err(_) => panic!("Error writing diary for today ('{}')!", iso_date),
        };
    }
}

fn get_base_folder() -> String {
    let cfg_str: String = read_file(CFG_PATH).expect("Should have been able to read the file");
    for line in cfg_str.trim().lines() {
        if line.trim().starts_with("DIARY_PATH=") {
            return expand_path(&line.trim().replace("DIARY_PATH=", "").to_string());
        }
    }
    panic!("Couldn't find the diary path in the cfg");
}

fn write_header_to_file(mut file: File, now: DateTime<Utc>) {
    let day_ordinal = Ordinal(now.day()).to_string();
    let weekday_name = now.format("%A").to_string();
    let month_name = Month::from_u32(now.month()).unwrap().name();
    let year = now.year();
    let extras: String = get_diary_extras();
    let file_contents = format!("# {weekday_name} {day_ordinal} {month_name} {year}\n\n\n{extras}");

    match file.write_all(file_contents.as_bytes()) {
        Ok(_) => println!("Successfuly wrote today's diary file"),
        Err(_) => panic!("Error writing today's diary file!"),
    };

    let base_folder = get_base_folder();
    let dir_to_open = base_folder.to_owned() + "/" + &now.year().to_string() + "-Diary/";
    Command::new("code").arg(dir_to_open).status().expect("Opening VS Code failed!");
}

fn get_diary_extras() -> String {
    return match read_file(EXTRAS_PATH) {
        Ok(text) => text,
        Err(_) => "".to_string(),
    };
}

fn expand_path(path: &str) -> String {
    return if path.starts_with("~/") {
        var("HOME").unwrap() + &path[1..]
    }else {path.to_string()};
}

fn read_file(path: &str) -> Result<String,std::io::Error> {
    let path_to_read = expand_path(path);
    return read_to_string(path_to_read);
}
