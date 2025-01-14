use derive_builder::Builder;

#[derive(Debug, Clone, Builder)]
pub struct Table {

    pub betting_limits: BettingLimits,

    /// How many decks does the table use.
    pub number_of_decks: u8,

    /// The percentage of cards dealt from the shoe before a shuffle.
    pub deck_penetration: f32,

    /// The payout ratio of a player's blackjack.
    #[builder(default = "3.0 / 2.0 ")]
    pub blackjack_payout: f32,

    /// The payout ratio of insurance.
    #[builder(default = "2.0 / 1.0 ")]
    pub insurance_payout: f32,

    /// Does the dealer hit on soft 17
    #[builder(default = "false")]
    pub h17: bool,

    /// Is double-after-split allowed?
    #[builder(default = "true")]
    pub das: bool,

    /// When can the player double down
    #[builder(default = "DoublingDownRule::DoubleOnHard9To11")]
    pub doubling_down: DoublingDownRule,

    /// When can the player surrender
    #[builder(default = "SurrenderRule::EarlySurrenderAgainst10")]
    pub surrendering: SurrenderRule,

    /// When does the dealer peek for blackjack
    #[builder(default = "DealerPeekingRule::ENHC")]
    pub peeking_for_blackjack: DealerPeekingRule,

    /// The maximum number of a times a player is allowed to split their hand.
    #[builder(default = "3")]
    pub max_splits: u8,

    /// Can the player resplit Aces?
    #[builder(default = "false")]
    pub can_resplit_aces: bool,

    /// Can the player play the hands from two split Aces?
    #[builder(default = "false")]
    pub can_play_split_aces: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DoublingDownRule {
    /// Doubling down is allowed on any two player cards.
    DoubleOnAny2,

    /// Doubling down is allowed only on totals between 9 and 11 (inclusive)
    DoubleOnHard9To11,

    /// Doubling down is allowed only on a hard total of 10 and 11.
    DoubleOnHard10And11,

    /// Doubling down is allowed only on hard or soft totals between 9 and 11 (inclusive).
    DoubleOnAny9To11,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SurrenderRule {
    /// The table does not offer surrendering.
    NoSurrender,

    /// The player can surrender if the dealer's face-up card is between 2 and 9 (inclusive).
    /// If the dealer's face-up card is a 10, the player can surrender before the dealer checks for blackjack.
    /// Surrender is not offered against a dealer's Ace.
    EarlySurrenderAgainst10,

    /// The player can surrender against any face-up card of the dealer before the dealer checks for blackjack.
    EarlySurrender,

    /// The option to surrender is offered only after the dealer checks for blackjack and is found not to have one.
    /// Essentially, the player can only surrender if the dealer's face-up card is between 2 and 9 (inclusive).
    LateSurrender,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DealerPeekingRule {
    /// European No Hole Card - the dealer does not receive a second card until all players have played their turn.
    /// This is slightly different from `NoPeek`, since the second card is dealt later.
    ENHC,

    /// The dealer is dealt a second card after dealing two cards to each player, but only looks at it after all players have played their turn.
    NoPeek,

    /// The dealer peeks for blackjack whenever their face-up card is a 10 or an Ace.
    Peek,

    /// The dealer peeks for blackjack only if their face-up card is an Ace.
    PeekOnAce,
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