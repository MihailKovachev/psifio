use super::{hand_total::HandTotalable, Card, HandTotal};

pub trait Hand: HandTotalable {
    fn hit(&mut self, card: Card);
    fn cards(&self) -> &[Card];

    fn is_blackjack(&self) -> bool {
        let cards = self.cards();

        cards.len() == 2 && cards.contains(&Card::Ace) && cards.contains(&Card::Ten)
    }
}

impl HandTotalable for [Card] {
    fn total(&self) -> HandTotal {
        let mut total: u8 = 0;
        let mut number_of_aces: u8 = 0;

        for card in self {
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