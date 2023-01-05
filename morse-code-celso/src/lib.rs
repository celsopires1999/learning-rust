#[derive(Debug)]
pub enum Pulse {
    Short,
    Long,
}

pub type Letter = Vec<Pulse>;

pub type Message = Vec<Letter>;

trait MorseCode {
    fn to_morse_code(&self) -> Message;
}

impl MorseCode for String {
    fn to_morse_code(&self) -> Message {
        use Pulse::*;
        let mut msg = Vec::with_capacity(self.len());
        for c in self.chars() {
            let pulse = match c {
                'A' | 'a' => vec![Short],
                'B' | 'b' => vec![Short, Long],
                'C' | 'c' => vec![Short, Long, Short],
                _ => continue,
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

pub fn print_morse_code(message: Message) {
    for letter in message {
        for pulse in letter {
            print!("{}", pulse);
        }
        print!(" ");
    }
    println!();
}

#[cfg(test)]
#[path = "tests.rs"] // nesse caso não precisamos do path, mas só deixei aqui para lembrar
mod tests;
