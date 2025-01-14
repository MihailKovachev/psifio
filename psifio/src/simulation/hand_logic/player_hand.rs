use crate::simulation::SimulatedPlayer;

use super::{Card, Hand, HandAction, HandTotal, HandTotalable};

#[derive(Debug)]
pub struct PlayerHand {
    pub player: * mut SimulatedPlayer,
    pub cards: Vec<Card>,
    pub history: Vec<HandAction>,
    pub bet: f32,
}

impl HandTotalable for PlayerHand {
    fn total(&self) -> HandTotal {
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
            if total + 11 > 21 {
                total += number_of_aces * 1;
                return HandTotal::new(total, false);
            } else {
                total += 11 + (number_of_aces - 1) * 1;
                return HandTotal::new(total, true);
            };
        }

        HandTotal::new(total, false)
    }
}

impl Hand for PlayerHand {
    fn hit(&mut self, card: Card) {
        debug_assert!(!self.was_doubled());

        self.cards.push(card);
    }
    
    fn is_blackjack(&self) -> bool {
        todo!()
    }

    fn cards(&self) -> &[Card] {
        &self.cards
    }
}

impl PlayerHand {
    pub fn double_down(&mut self, card: Card) {
        self.cards.push(card);
    }

    pub fn split(&mut self) -> PlayerHand {
        debug_assert!(self.cards.len() == 2);
        debug_assert!(self.cards[0] == self.cards[1]);

        let card = self.cards.pop().unwrap();

        PlayerHand {
            player: self.player,
            cards: vec![card],
            history: vec![HandAction::Split],
            bet: self.bet
        }
    }

    pub fn was_split(&self) -> bool {
        self.history.contains(&HandAction::Split)
    }

    pub fn was_doubled(&self) -> bool {
        self.history.last() == Some(&HandAction::DoubleDown)
    }

    pub fn was_insured(&self) -> bool {
        todo!()
    }

    pub fn cards(&self) -> &[Card] {
        &self.cards
    }
}