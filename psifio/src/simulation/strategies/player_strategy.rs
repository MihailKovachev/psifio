use crate::{simulation::hand_logic::{HandAction, InsuranceAction, PlayerHand}, table::BettingLimits};

pub trait PlayerStrategy: core::fmt::Debug {

    /// How many hands the player wants to play this round.
    fn hands_to_play(&self) -> u64;

    /// The bet to place on the current hand.
    /// "None" only if hands_to_play is zero.
    fn bet(&self, betting_limits: BettingLimits) -> Option<f32>;

    /// What to do if insurance is offered.
    fn on_insurance_offered(&self) -> InsuranceAction;

    /// What action to take on the current hand.
    fn play_hand(&mut self, hand: &PlayerHand) -> HandAction;
}