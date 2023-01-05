#[derive(Debug, PartialEq)]
enum Pulse {
    Short,
    Long,
}

/// Represents a single character
type Letter = Vec<Pulse>;

/// Represents a string of characters
type Message = Vec<Letter>;

trait MorseCode {
    fn to_morse_code(&self) -> Message;
}

impl MorseCode for String {
    fn to_morse_code(&self) -> Message {
        use Pulse::*;

        let mut msg = Vec::with_capacity(self.len());

        for c in self.chars() {
            let pulse = match c {
                'A' | 'a' => vec![Short, Long],
                'B' | 'b' => vec![Long, Short, Long, Short],
                _ => vec![Long, Long, Long],
                // table to be completed
                // _ => continue;
            };
            msg.push(pulse);
        }
        msg
    }
}

impl std::fmt::Display for Pulse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Pulse::Short => write!(f, "."),
            Pulse::Long => write!(f, "_"),
        }
    }
}

fn print_morse_code(code: Message) {
    for letter in code.iter() {
        for pulse in letter.iter() {
            print!("{}", pulse);
        }
        print!(" ");
    }
    println!();
}

#[cfg(test)]
#[path = "tests.rs"] // nesse caso não precisamos do path, mas só deixei aqui para lembrar
mod tests;
