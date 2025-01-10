#[derive(Debug, Copy, Clone)]
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
    Ace = 11
}

impl From<u8> for Card {
    fn from(value: u8) -> Self {
        assert!(value >= 2);
        assert!(value <= 11);

        unsafe { std::mem::transmute(value) }
    }
}

impl From<Card> for u8 {
    fn from(value: Card) -> Self {
        unsafe { std::mem::transmute(value) }
    }
}