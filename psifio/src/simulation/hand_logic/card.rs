#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum Card {
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
    Nine = 9,
    Ten = 10,
    Ace = 11,
}

pub const CARDS: [Card; 10] = [
    Card::Two,
    Card::Three,
    Card::Four,
    Card::Five,
    Card::Six,
    Card::Seven,
    Card::Eight,
    Card::Nine,
    Card::Ten,
    Card::Ace,
];

impl From<u8> for Card {
    fn from(value: u8) -> Self {
        debug_assert!(value >= 2);
        debug_assert!(value <= 11);

        unsafe { std::mem::transmute(value) }
    }
}

impl From<Card> for u8 {
    fn from(value: Card) -> Self {
        unsafe { std::mem::transmute(value) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn u8_from_card() {
        assert_eq!(u8::from(Card::Two), 2);
        assert_eq!(u8::from(Card::Three), 3);
        assert_eq!(u8::from(Card::Four), 4);
        assert_eq!(u8::from(Card::Five), 5);
        assert_eq!(u8::from(Card::Six), 6);
        assert_eq!(u8::from(Card::Seven), 7);
        assert_eq!(u8::from(Card::Eight), 8);
        assert_eq!(u8::from(Card::Nine), 9);
        assert_eq!(u8::from(Card::Ten), 10);
        assert_eq!(u8::from(Card::Ace), 11);
    }

    #[test]
    fn card_from_u8() {
        assert_eq!(Card::from(2), Card::Two);
        assert_eq!(Card::from(3), Card::Three);
        assert_eq!(Card::from(4), Card::Four);
        assert_eq!(Card::from(5), Card::Five);
        assert_eq!(Card::from(6), Card::Six);
        assert_eq!(Card::from(7), Card::Seven);
        assert_eq!(Card::from(8), Card::Eight);
        assert_eq!(Card::from(9), Card::Nine);
        assert_eq!(Card::from(10), Card::Ten);
        assert_eq!(Card::from(11), Card::Ace);
    }
}
