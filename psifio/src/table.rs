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

    pub fn betting_limits(&self) -> BettingLimits {
        self.betting_limits
    }

    pub fn table_rules(&self) -> &TableRules {
        &self.table_rules
    }

    pub fn players(&self) -> &Vec<Player> {
        &self.players
    }
}

#[derive(Debug, Clone, Copy)]
pub struct BettingLimits {
    pub min: f32,
    pub max: f32
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