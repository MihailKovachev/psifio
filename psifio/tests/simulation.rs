

#[cfg(test)]
mod tests {
    use psifio::table::{BettingLimits, DealerPeekingRule, DoublingDownRule, SurrenderRule, Table};


    #[test]
    fn simulate_shoe() {
        let table = Table {
            betting_limits: BettingLimits {min: 10.0, max: 1000.0 },
            number_of_decks: 6,
            deck_penetration: 0.80,
            blackjack_payout: 3.0 / 2.0,
            insurance_payout: 2.0 / 1.0,
            h17: false,
            das: true,
            doubling_down: DoublingDownRule::DoubleOnAny2,
            surrendering: SurrenderRule::NoSurrender,
            peeking_for_blackjack: DealerPeekingRule::Peek,
            max_splits: 3,
            can_resplit_aces: false,
            can_play_split_aces: false,
        };

        
    }
}