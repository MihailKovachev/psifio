use crate::{simulation::hand_logic::{HandAction, InsuranceAction, PlayerHand}, table::BettingLimits};

pub trait PlayerStrategy: core::fmt::Debug {
    fn hands_to_play(&self) -> usize;
    fn bet(&self, betting_limits: BettingLimits) -> Option<f32>;
    fn on_insurance_offered(&self) -> InsuranceAction;
    fn play_hand(&mut self, hand: &PlayerHand) -> HandAction;
}