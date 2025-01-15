use rand::seq::SliceRandom;

use super::hand_logic::Card;

#[derive(Debug)]
pub struct Shoe {
    cards: Vec<Card>,
    cards_drawn: usize,
}

impl Shoe {
    pub fn new_ordered_shoe(number_of_decks: u8) -> Self {
        let mut cards: Vec<Card> = Vec::with_capacity(number_of_decks as usize * 52);
        for _i in 0..number_of_decks {
            cards.extend(new_deck());
        }

        Shoe {
            cards,
            cards_drawn: 0,
        }
    }

    pub fn new_shuffled_shoe(number_of_decks: u8) -> Self {
        let mut shoe = Self::new_ordered_shoe(number_of_decks);

        shoe.cards.shuffle(&mut rand::thread_rng());

        shoe
    }

    pub fn shuffle(&mut self) {
        self.cards.shuffle(&mut rand::thread_rng());
        self.cards_drawn = 0;
    }

    pub fn draw_card(&mut self) -> Card {
        self.cards_drawn += 1;

        self.cards[self.cards_drawn - 1]
    }

    pub fn cards_drawn(&self) -> usize {
        self.cards_drawn
    }
}

/// Returns an order 52-card deck
const fn new_deck() -> [Card; 52] {
    [
        Card::Two,
        Card::Three,
        Card::Four,
        Card::Five,
        Card::Six,
        Card::Seven,
        Card::Eight,
        Card::Nine,
        Card::Ten,
        Card::Ten,
        Card::Ten,
        Card::Ten,
        Card::Ace,
        Card::Two,
        Card::Three,
        Card::Four,
        Card::Five,
        Card::Six,
        Card::Seven,
        Card::Eight,
        Card::Nine,
        Card::Ten,
        Card::Ten,
        Card::Ten,
        Card::Ten,
        Card::Ace,
        Card::Two,
        Card::Three,
        Card::Four,
        Card::Five,
        Card::Six,
        Card::Seven,
        Card::Eight,
        Card::Nine,
        Card::Ten,
        Card::Ten,
        Card::Ten,
        Card::Ten,
        Card::Ace,
        Card::Two,
        Card::Three,
        Card::Four,
        Card::Five,
        Card::Six,
        Card::Seven,
        Card::Eight,
        Card::Nine,
        Card::Ten,
        Card::Ten,
        Card::Ten,
        Card::Ten,
        Card::Ace,
    ]
}
