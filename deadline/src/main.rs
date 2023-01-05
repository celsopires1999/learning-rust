use chrono::{DateTime, Local, TimeZone};
use deadline::{Deadline, ImportantEvent};

fn main() {
    let christmas: DateTime<Local> = Local.with_ymd_and_hms(2023, 12, 25, 0, 0, 0).unwrap();
    let missed_christmas = ImportantEvent::new(christmas, "Christmas 2025".to_string());

    if missed_christmas.is_passed() {
        println!("Pity");
    } else {
        println!("Ooo hohoho");
    }
}
