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

    hand_stats_table: HandStatsTable
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

    
    /// Returns a reference to the hand statistics table.
    pub fn get_hand_stats(&self) -> &HandStatsTable {
        &self.hand_stats_table
    }

    /// Returns a mutable reference to the hand statistics table.
    pub fn get_hand_stats_mut(&mut self) -> &mut HandStatsTable {
        &mut self.hand_stats_table
    }
}