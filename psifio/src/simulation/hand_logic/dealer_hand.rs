use super::{hand_total::HandTotalable, Card, Hand, HandTotal};

#[derive(Debug)]
pub struct DealerHand {
    /// The cards of the hand
    cards: Vec<Card>
}

impl DealerHand {
    pub fn new() -> Self {
        Self {
            cards: Vec::with_capacity(4),
        }
    }
}

impl Hand for DealerHand{

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