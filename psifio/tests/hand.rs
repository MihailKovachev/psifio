#[cfg(test)]
mod tests {
    use psifio::{card::Card, hand::Hand};

    #[test]
    fn hand() {
        let hand = Hand::with_initial_cards(vec![Card::Two, Card::Ten, Card::Three]);
        println!("{:#?}", hand);
        assert_eq!(hand.total(), 15);

        let hand = Hand::with_initial_cards(vec![Card::Two, Card::Ace]);
        assert_eq!(hand.total(), 13);

        let hand = Hand::with_initial_cards(vec![Card::Ace, Card::Ace]);
        assert_eq!(hand.total(), 12);

        let hand = Hand::with_initial_cards(vec![Card::Ace, Card::Ten]);
        assert_eq!(hand.total(), 21);

        let hand = Hand::with_initial_cards(vec![Card::Nine, Card::Six, Card::Ace]);
        assert_eq!(hand.total(), 16);

        let hand = Hand::with_initial_cards(vec![Card::Ace, Card::Six, Card::Nine]);
        assert_eq!(hand.total(), 16);
    }
}