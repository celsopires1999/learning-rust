#[derive(Debug, PartialEq)]
enum Card {
    Ace,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Jack,
    Queen,
    King,
}
const MAX_HAND: usize = 21;

struct Hand {
    cards: Vec<Card>,
}

impl Hand {
    fn new() -> Self {
        Hand { cards: vec![] }
    }

    fn add(&mut self, card: Card) {
        self.cards.push(card);
    }

    fn value(&self) -> usize {
        const ACES_WEIGHT: usize = 11;
        let mut subtotal = 0;
        let mut aces_seen = 0;

        for card in self.cards.iter() {
            use Card::*;
            subtotal += match card {
                Ace => {
                    aces_seen += 1;
                    0
                }
                One => 1,
                Two => 2,
                Three => 3,
                Four => 4,
                Five => 5,
                Six => 6,
                Seven => 7,
                Eight => 8,
                Nine => 9,
                Jack | Queen | King => 10,
            };
        }

        for _ in 0..aces_seen {
            let aces_value = if subtotal == MAX_HAND { 1 } else { ACES_WEIGHT };
            subtotal += aces_value;
        }

        subtotal
    }

    fn is_loosing_hand(&self) -> bool {
        self.value() > MAX_HAND
    }
}

#[cfg(test)]
mod tests;
