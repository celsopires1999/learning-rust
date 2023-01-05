use chrono::TimeZone;

use super::*;

#[test]
fn it_should_create_an_important_event() {
    let date_now = Local::now();
    let event = ImportantEvent::new(date_now, "Birthday".to_string());
    assert_eq!(event.when, date_now);
    assert_eq!(event.what, "Birthday".to_string());
}

#[test]
fn it_should_be_passed() {
    let birthday: DateTime<Local> = Local.with_ymd_and_hms(2022, 12, 05, 0, 0, 0).unwrap();
    let event = ImportantEvent::new(birthday, "Birthday".to_string());
    assert!(event.is_passed())
}

#[test]
fn it_should_not_be_passed() {
    let birthday: DateTime<Local> = Local.with_ymd_and_hms(2024, 12, 05, 0, 0, 0).unwrap();
    let event = ImportantEvent::new(birthday, "Birthday".to_string());
    assert!(!event.is_passed())
}
