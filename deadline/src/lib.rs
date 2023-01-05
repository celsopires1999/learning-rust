use chrono::{DateTime, Local};
pub trait Deadline {
    fn is_passed(&self) -> bool;
}
pub struct ImportantEvent {
    pub when: DateTime<Local>,
    pub what: String,
}

impl ImportantEvent {
    pub fn new(when: DateTime<Local>, what: String) -> Self {
        ImportantEvent { when, what }
    }

    pub fn show_what(&self) -> String {
        self.what.to_owned()
    }
}

impl Deadline for ImportantEvent {
    fn is_passed(&self) -> bool {
        self.when < Local::now()
    }
}

#[cfg(test)]
mod tests;
