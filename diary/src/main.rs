use chrono::prelude::{Date, Utc};


fn main() {
    let now: Date<Utc> = Utc::now();
    println!("{}", now.format());
}

