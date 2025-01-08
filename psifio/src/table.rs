use crate::{player::Player, table_rules::TableRules};

pub struct Table {

    betting_limits: BettingLimits,
    table_rules: TableRules,
    players: Vec<Player>
}

impl Table {
    pub fn new(betting_limits: BettingLimits, table_rules: TableRules, players: Vec<Player>) -> Self {
        Self {
            betting_limits, table_rules, players
        }
    }
}

#[derive(Debug)]
pub struct BettingLimits {
    min: f32,
    max: f32
}

impl BettingLimits {
    pub fn new(min: f32, max: f32) -> Self {

        assert!(min < max);

        Self { min, max }
    }
}

impl Default for BettingLimits {
    fn default() -> Self {
        Self {
            min: 100.0,
            max: 1000.0
        }
    }
}