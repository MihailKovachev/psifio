use std::collections::HashMap;

#[derive(Debug)]
pub struct Stats {
    pub wins: HashMap<WinReason, u64>,
    pub losses: HashMap<LossReason, u64>,
    pub pushes: HashMap<PushType, u64>
}

impl Stats {
    pub fn new() -> Self {

        let mut wins = HashMap::with_capacity(2);
        wins.insert(WinReason::PlayerBlackjack, 0);
        wins.insert(WinReason::HigherTotal, 0);

        let mut losses = HashMap::with_capacity(4);
        losses.insert(LossReason::DealerBlackjack, 0);
        losses.insert(LossReason::Busted, 0);
        losses.insert(LossReason::LowerTotal, 0);
        losses.insert(LossReason::Surrendered, 0);

        let mut pushes = HashMap::with_capacity(2);
        pushes.insert(PushType::Blackjack, 0);
        pushes.insert(PushType::NonBlackjack, 0);

        Self {
            wins,
            losses,
            pushes
        }
    }
}

/// Represents the reason why the player's hand lost.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LossReason {
    /// The dealer had a blackjack and the player did not.
    DealerBlackjack,

    /// The player busted.
    Busted,

    /// Neither the player nor the dealer busted, but the dealer's hand had a higher total.
    LowerTotal,

    /// The player surrendered
    Surrendered
}

/// Represents the reason why the player's hand won.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum WinReason {
    /// The player had a blackjack and the dealer did not.
    PlayerBlackjack,

    /// Neither the player nor the dealer busted, and the player's hand had a higher total.
    HigherTotal
}

/// Specifies the type of push which occurred.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PushType {
    /// Both the player and the dealer had a blackjack
    Blackjack,

    /// Neither the player nor the dealer had a blackjack.
    /// Neither the player nor the dealer busted.
    /// The player and the dealer had the same total.
    NonBlackjack
}

#[derive(Debug)]
pub struct DealerBlackjackStats {
    pub pushed: u64,
    pub lost: u64
}

impl DealerBlackjackStats {
    pub fn new() -> Self {
        Self {
            lost: 0,
            pushed: 0
        }
    }
}