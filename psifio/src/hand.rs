use crate::card::Card;

#[derive(Debug)]
pub struct Hand {
    /// The cards of the hand
    cards: Vec<Card>,
}

impl Hand {
    pub fn new() -> Self {
        Self {
            cards: Vec::with_capacity(4),
        }
    }

    pub fn with_initial_cards(cards: Vec<Card>) -> Self {
        Self { cards }
    }

    pub fn total(&self) -> u8 {
        let mut total: u8 = 0;
        let mut number_of_aces: u8 = 0;

        for card in &self.cards {
            match card {
                Card::Ace => {
                    number_of_aces += 1;
                }
                _ => {
                    total += u8::from(card.clone());
                }
            }
        }

        if number_of_aces > 0 {
            total = if total + 11 > 21 {
                total + number_of_aces * 1
            } else {
                total + 11 + (number_of_aces - 1) * 1
            };
        }

        total
    }

    pub fn add_card(&mut self, card: Card) {
        self.cards.push(card);
    }
}
