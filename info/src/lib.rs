// use std::fmt::Display;

pub fn info<T: AsRef<str>>(text: &T) {
    println!("{}", text.as_ref());
}

// pub fn info<T: ToString>(text: &T) {
//     println!("{}", text.to_string());
// }

// pub fn info<T: Display>(text: &T) {
//     println!("{}", text);
// }

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
