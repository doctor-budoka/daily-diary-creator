use std::fs::{File, OpenOptions, create_dir_all};
use std::io::Write;
use std::path::Path;
use num_traits::cast::FromPrimitive;
use chrono::prelude::{DateTime, Utc, Datelike, Month};
use ordinal::Ordinal;

const BASE_FOLDER: &str = "/Users/james/Dropbox/";

fn main() {
    let now: DateTime<Utc> = Utc::now();
    let iso_date = now.format("%Y-%m-%d").to_string();
    let file_name = iso_date.to_owned() + ".md";
    let target_dir = BASE_FOLDER.to_owned() + "/" + &now.year().to_string() + "-Diary/";
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

fn write_header_to_file(mut file: File, now: DateTime<Utc>) {
    let day_ordinal = Ordinal(now.day()).to_string();
    let weekday_name = now.format("%A").to_string();
    let month_name = Month::from_u32(now.month()).unwrap().name();
    let year = now.year();
    let file_contents = format!("# {weekday_name} {day_ordinal} {month_name} {year}\n\n\n");

    match file.write_all(file_contents.as_bytes()) {
        Ok(_) => println!("Successfuly wrote today's diary file"),
        Err(_) => panic!("Error writing today's diary file!"),
    };
}
