use super::hand_logic::{Card, DealerHand, PlayerHand};

#[derive(Debug)]
pub struct Round {
    pub player_hands: Vec<PlayerHand>,
    pub dealer_hand: DealerHand
}

#[derive(Debug)]
pub enum PlayerHandOutcome {
    Win,

    Push,

    Loss
}