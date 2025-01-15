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

#[cfg(test)]
mod tests {
    use crate::simulation::hand_logic::{Card, HandTotalable};

    #[test]
    fn hand_total() {
        let cards = [Card::Ace, Card::Ten];
        assert_eq!(cards.total().value(), 21);
        assert!(cards.total().is_soft());

        let cards = [Card::Ten, Card::Ace];
        assert_eq!(cards.total().value(), 21);
        assert!(cards.total().is_soft());

        let cards = [Card::Three, Card::Ace];
        assert_eq!(cards.total().value(), 14);
        assert!(cards.total().is_soft());

        let cards = [Card::Three, Card::Two, Card::Ace];
        assert_eq!(cards.total().value(), 16);
        assert!(cards.total().is_soft());

        let cards = [Card::Three, Card::Ace, Card::Two];
        assert_eq!(cards.total().value(), 16);
        assert!(cards.total().is_soft());

        let cards = [Card::Ace, Card::Three, Card::Two];
        assert_eq!(cards.total().value(), 16);
        assert!(cards.total().is_soft());

        let cards = [Card::Ace, Card::Ace];
        assert_eq!(cards.total().value(), 12);
        assert!(cards.total().is_soft());

        let cards = [Card::Ace, Card::Ace, Card::Ace];
        assert_eq!(cards.total().value(), 13);
        assert!(cards.total().is_soft());

        let cards = [Card::Ace, Card::Nine, Card::Five];
        assert_eq!(cards.total().value(), 15);
        assert!(!cards.total().is_soft());

        let cards = [Card::Nine, Card::Ace, Card::Five];
        assert_eq!(cards.total().value(), 15);
        assert!(!cards.total().is_soft());

        let cards = [Card::Nine, Card::Five, Card::Ace];
        assert_eq!(cards.total().value(), 15);
        assert!(!cards.total().is_soft());

        let cards = [Card::Two, Card::Eight];
        assert_eq!(cards.total().value(), 10);
        assert!(!cards.total().is_soft());
    }
}
