#[derive(Debug)]
pub struct Player {
    balance: f32,
}

impl Player {
    pub fn new(initial_bankroll: f32) -> Result<Self, PlayerError> {
        if initial_bankroll < 0.0 {
            return Err(PlayerError::BankrollCannotBeNegative);
        } else {
            Ok(Self { balance: initial_bankroll })
        }
    }
}

pub enum PlayerError {
    BankrollCannotBeNegative
}