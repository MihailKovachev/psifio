use crate::simulation::hand_logic::HandTotal;

use super::HandStats;

/// A table containing statistics data about every possible player hand total.
#[derive(Debug)]
pub struct HandStatsTable {
    stats: [HandStats; 28],
}

impl HandStatsTable {
    pub fn new() -> Self {
        Self {
            stats: [
                // Hard totals
                HandStats::new(4),
                HandStats::new(5),
                HandStats::new(6),
                HandStats::new(7),
                HandStats::new(8),
                HandStats::new(9),
                HandStats::new(10),
                HandStats::new(11),
                HandStats::new(12),
                HandStats::new(13),
                HandStats::new(14),
                HandStats::new(15),
                HandStats::new(16),
                HandStats::new(17),
                HandStats::new(18),
                HandStats::new(19),
                HandStats::new(20),
                HandStats::new(21),

                // Soft totals
                HandStats::new(12),
                HandStats::new(13),
                HandStats::new(14),
                HandStats::new(15),
                HandStats::new(16),
                HandStats::new(17),
                HandStats::new(18),
                HandStats::new(19),
                HandStats::new(20),
                HandStats::new(21),
            ],
        }
    }

    pub fn get_stats_for_total(&self, total: HandTotal) -> &HandStats {
        if total.is_soft() {
            &self.stats[total.value() as usize + 6]
        } else {
            &self.stats[total.value() as usize - 4]
        }
    }

    pub fn get_stats_for_total_mut(&mut self, total: HandTotal) -> &mut HandStats {
        if total.is_soft() {
            &mut self.stats[total.value() as usize + 6]
        } else {
            &mut self.stats[total.value() as usize - 4]
        }
    }
}
