use rand::Rng;

struct Passwords {
    length: usize,
}

impl Passwords {
    fn new() -> Self {
        Self::with_length(10)
    }

    fn with_length(length: usize) -> Self {
        Passwords { length }
    }
}

impl IntoIterator for Passwords {
    type Item = String;

    type IntoIter = PasswordsIterator;

    fn into_iter(self) -> Self::IntoIter {
        PasswordsIterator {
            length: self.length,
        }
    }
}

struct PasswordsIterator {
    length: usize,
}

impl Iterator for PasswordsIterator {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let mut result = String::with_capacity(self.length);
        for _ in 0..self.length {
            result.push((b'a' + rand::thread_rng().gen_range(0..=(b'z' - b'a'))) as char);
        }

        Some(result)
    }
}

fn main() {
    for p in Passwords::new().into_iter().take(3) {
        println!("The next password is {}", p);
    }

    Passwords::with_length(5)
        .into_iter()
        .take(3)
        .for_each(|p| println!("The next password is {}", p));
}
