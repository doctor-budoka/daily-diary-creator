use num_traits::cast::FromPrimitive;
use chrono::prelude::{DateTime, Utc, Datelike, Month};
use ordinal::Ordinal;



fn main() {
    let now: DateTime<Utc> = Utc::now();
    let file_name = now.format("%Y-%m-%d").to_string() + ".md";

    let mut file = OpenOptions::new()
    .create_new(true)
    .write(true)
    .append(true)
    .open(file_name)
    .unwrap();


    let day_ordinal = Ordinal(now.day()).to_string();
    let weekday_name = now.format("%A").to_string();
    let month_name = Month::from_u32(now.month()).unwrap().name();
    let year = now.year();
    let file_contents = format!("# {weekday_name} {day_ordinal} {month_name} {year}\n\n\n");

    println!("{}", file_name);
    println!("'{}'", file_contents);
}

