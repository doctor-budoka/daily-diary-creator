use chrono::prelude::{DateTime, Utc};


fn main() {
    let now: DateTime<Utc> = Utc::now();
    let file_name = now.format("%Y-%m-%d").to_string() + ".md";
    println!("{}", file_name);
    println!("{}", now.format("%A %-d %B %Y"));
}

