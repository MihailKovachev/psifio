use super::{hand_total::HandTotalable, Card, Hand, HandTotal};

#[derive(Debug)]
pub struct DealerHand {
    /// The cards of the hand
    cards: Vec<Card>,
}

impl DealerHand {
    pub fn new() -> Self {
        Self {
            cards: Vec::with_capacity(4),
        }
    }

    pub fn with_initial_cards(cards: Vec<Card>) -> Self {
        Self { cards }
    }
}

impl Hand for DealerHand {
    fn hit(&mut self, card: Card) {
        self.cards.push(card);
    }

    fn is_blackjack(&self) -> bool {
        self.cards.len() == 2 && self.cards.contains(&Card::Ace) && self.cards.contains(&Card::Ten)
    }

    fn cards(&self) -> &[Card] {
        &self.cards
    }
}

impl HandTotalable for DealerHand {
    fn total(&self) -> HandTotal {
        self.cards.total()
    }
}

#[cfg(test)]
mod tests {
    use crate::simulation::hand_logic::card::CARDS;

    use super::*;

    #[test]
    fn is_blackjack() {
        // Blackjacks
        assert!(DealerHand::with_initial_cards(vec![Card::Ace, Card::Ten]).is_blackjack());
        assert!(DealerHand::with_initial_cards(vec![Card::Ten, Card::Ace]).is_blackjack());

        // No blackjacks
        for first_card in &CARDS[0..9] {
            for second_card in &CARDS[0..9] {
                assert!(!DealerHand::with_initial_cards(vec![
                    first_card.clone(),
                    second_card.clone()
                ])
                .is_blackjack());
            }
        }
    }
}
