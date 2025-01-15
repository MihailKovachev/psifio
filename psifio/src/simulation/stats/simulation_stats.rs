use super::{BlackjackStats, HandStatsTable};

#[derive(Debug)]
pub struct SimulationStats {
    /// The total number of shoes which were simulated.
    shoes_played: u64,

    /// The total number of rounds which were simulated.
    rounds_played: u64,

    /// The total number of player hands which were simulated.
    hands_played: u64,

    pub blackjack_stats: BlackjackStats,

    pub hand_stats_table: HandStatsTable
}

impl SimulationStats {
    pub fn new() -> Self {
        Self {
            shoes_played: 0,
            rounds_played: 0,
            hands_played: 0,

            blackjack_stats: BlackjackStats::new(),

            hand_stats_table: HandStatsTable::new()
            
        }
    }
}