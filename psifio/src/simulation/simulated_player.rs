use crate::simulation::PlayerStrategy;

#[derive(Debug)]
pub struct SimulatedPlayer {
    pub bankroll: f32,
    pub strategy: Box<dyn PlayerStrategy>,

    pub remaining_splits: u8,
    current_balance: f32,
}

impl SimulatedPlayer {
    pub fn new(bankroll: f32, strategy: Box<dyn PlayerStrategy>, max_splits: u8) -> Self {

        assert!(bankroll > 0.0);

        Self { bankroll, strategy, current_balance: bankroll, remaining_splits: max_splits }
    }
}