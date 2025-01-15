use crate::{
    simulation::{hand_logic::HandTotal, PlayerStrategy, WinReason},
    table::{DealerPeekingRule, Table},
};

use super::{
    hand_logic::{Card, DealerHand, Hand, HandAction, HandTotalable, PlayerHand}, LossReason, PushType, Round, Shoe, SimulatedPlayer,
    SimulationStats,
};

pub struct Simulation {
    table: Table,
    players: Vec<SimulatedPlayer>,
}

impl Simulation {
    pub fn run(&mut self) {
        todo!()
    }

    pub fn simulate_shoe(&mut self) {
        let mut shoe = Shoe::new_ordered_shoe(self.table.number_of_decks);

        let mut stats = SimulationStats::new();

        loop {
            self.simulate_round(&mut shoe, &mut stats);

            if (shoe.cards_drawn() as f32 / ((self.table.number_of_decks * 52) as f32))
                > self.table.deck_penetration
            {
                break;
            }
        }
    }

    pub fn simulate_round(&mut self, shoe: &mut Shoe, stats: &mut SimulationStats) {
        let number_of_initial_player_hands = {
            let mut total = 0;
            for player in &self.players {
                total += player.strategy.hands_to_play()
            }
            total
        };

        let mut player_hands: Vec<PlayerHand> =
            Vec::with_capacity(number_of_initial_player_hands as usize);

        // Deal the first card of each hand for all players
        for player in &mut self.players {
            for _ in 0..player.strategy.hands_to_play() {
                player_hands.push(PlayerHand::new(
                    player,
                    vec![shoe.draw_card()],
                    player.strategy.bet(self.table.betting_limits).unwrap(),
                ));
            }
        }

        // Deal the dealer's face-up card
        let mut dealer_hand = DealerHand::new();
        dealer_hand.hit(shoe.draw_card());

        // Deal the second card of each hand for all players
        for player_hand in &mut player_hands {
            player_hand.hit(shoe.draw_card());
        }

        let mut round = Round {
            dealer_hand,
            player_hands,
        };

        // Dealer peeking logic
        match self.table.peeking_for_blackjack {
            DealerPeekingRule::Peek => {
                round.dealer_hand.hit(shoe.draw_card());

                if round.dealer_hand.is_blackjack() {
                    Self::evaluate_round(&round, stats);
                    return; // The round ends
                }
            }
            DealerPeekingRule::PeekOnAce => {
                round.dealer_hand.hit(shoe.draw_card());

                if round.dealer_hand.cards()[0] == Card::Ace {
                    Self::evaluate_round(&round, stats);
                    return; // Round ends
                }
            }

            DealerPeekingRule::NoPeek => {
                round.dealer_hand.hit(shoe.draw_card());
            }

            DealerPeekingRule::ENHC => {}
        }

        {
            let mut total_player_hands = round.player_hands.len();
            let mut i = 0;
            while i < total_player_hands {
                loop {
                    let player_action = unsafe {
                        (*round.player_hands[i].player)
                            .strategy
                            .play_hand(&round.player_hands[i])
                    };

                    match player_action {
                        HandAction::Hit => {
                            round.player_hands[i].hit(shoe.draw_card());
                        }

                        HandAction::DoubleDown => {
                            if round.player_hands[i].was_doubled() {
                                break;
                            }

                            if !self.table.das && round.player_hands[i].was_split() {
                                break;
                            }

                            round.player_hands[i].double_down(shoe.draw_card());
                        }

                        HandAction::Split => {
                            unsafe {
                                if (*round.player_hands[i].player).remaining_splits <= 0 {
                                    break;
                                }
                            }

                            let second_hand =
                                round.player_hands[i].split([shoe.draw_card(), shoe.draw_card()]);

                            round.player_hands.insert(i + 1, second_hand);

                            unsafe {
                                (*round.player_hands[i].player).remaining_splits -= 1;
                            }
                            total_player_hands += 1;
                        }

                        HandAction::Surrender => {
                            round.player_hands[i].surrender();
                            break;
                        }

                        HandAction::Stand => {
                            round.player_hands[i].stand();
                            break;
                        }
                    }

                    i += 1;
                }
            }
        }

        if self.table.peeking_for_blackjack == DealerPeekingRule::ENHC {
            round.dealer_hand.hit(shoe.draw_card());
        }

        let mut dealer_total = round.dealer_hand.total();

        while dealer_total.value() < 17 {
            round.dealer_hand.hit(shoe.draw_card());
            dealer_total = round.dealer_hand.total();

            // Hit soft 17
            if dealer_total.value() == 17 && self.table.h17 && dealer_total.is_soft() {
                round.dealer_hand.hit(shoe.draw_card());
            }
        }

        Self::evaluate_round(&round, stats);
    }

    pub fn add_player(&mut self, bankroll: f32, strategy: Box<dyn PlayerStrategy>) {
        self.players.push(SimulatedPlayer::new(
            bankroll,
            strategy,
            self.table.max_splits,
        ));
    }

    fn evaluate_round(round: &Round, stats: &mut SimulationStats) {
        fn update_stats_for_action_on_total(
            total: HandTotal,
            action: Option<HandAction>,
            win_reason: Option<WinReason>,
            push_type: Option<PushType>,
            loss_reason: Option<LossReason>,
            stats: &mut SimulationStats,
        ) {
            match (win_reason, push_type, loss_reason) {
                (Some(win_reason), None, None) => {
                    *stats
                        .hand_stats_table
                        .get_stats_for_total_mut(total)
                        .get_stats_for_action_mut(action)
                        .wins
                        .get_mut(&win_reason)
                        .unwrap() += 1;
                }
                (None, Some(push_type), None) => {
                    *stats
                        .hand_stats_table
                        .get_stats_for_total_mut(total)
                        .get_stats_for_action_mut(action)
                        .pushes
                        .get_mut(&push_type)
                        .unwrap() += 1;
                }
                (None, None, Some(loss_reason)) => {
                    *stats
                        .hand_stats_table
                        .get_stats_for_total_mut(total)
                        .get_stats_for_action_mut(action)
                        .losses
                        .get_mut(&loss_reason)
                        .unwrap() += 1;
                }

                _ => {
                    panic!("Exactly one of win_reason, push_type or loss_reason must be Some.");
                }
            }
        }

        let dealer_blackjack = round.dealer_hand.is_blackjack();
        if dealer_blackjack {
            stats.blackjack_stats.dealer_blackjacks += 1;
        }
    }
}

pub fn new(table: Table) -> Simulation {
    Simulation {
        table,
        players: Vec::new(),
    }
}
