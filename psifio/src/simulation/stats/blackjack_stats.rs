#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BlackjackStats {
    // How many times did the dealer have a blackjack
    pub dealer_blackjacks: u64,

    // How many times did a player have a blackjack
    pub player_blackjacks: u64,
}

impl BlackjackStats {
    pub fn new() -> Self {
        Self {
            dealer_blackjacks: 0,
            player_blackjacks: 0,
        }
    }
}