use std::collections::HashMap;

use crate::simulation::hand_logic::HandAction;

use super::Stats;

/// A type to store all winrate statistics about a specific player hand total.
#[derive(Debug)]
pub struct HandStats {
    pub total: u8,

    /// The stats for each possible player action on the given total.
    /// None is used for when the player did not have a chance to play their hand such as when the dealer peeked and had a blackjack.
    stats: HashMap<Option<HandAction>, Stats>,
}

impl HandStats {
    pub fn new(total: u8) -> Self {
        let mut stats = HashMap::with_capacity(6);
        stats.insert(None, Stats::new());
        stats.insert(Some(HandAction::Stand), Stats::new());
        stats.insert(Some(HandAction::Hit), Stats::new());
        stats.insert(Some(HandAction::DoubleDown), Stats::new());
        stats.insert(Some(HandAction::Split), Stats::new());
        stats.insert(Some(HandAction::Surrender), Stats::new());

        Self { total, stats }
    }

    pub fn get_stats_for_action(&self, action: &Option<HandAction>) -> &Stats {
        self.stats.get(action).unwrap()
    }

    pub fn get_stats_for_action_mut(&mut self, action: &Option<HandAction>) -> &mut Stats {
        self.stats.get_mut(action).unwrap()
    }
}
