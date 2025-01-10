use derive_builder::Builder;

#[derive(Debug, Clone, Builder)]
pub struct TableRules {

    /// How many decks does the table use.
    number_of_decks: u8,

    /// The percentage of cards dealt from the shoe before a shuffle.
    deck_penetration: f32,

    /// The payout ratio of a player's blackjack.
    #[builder(default = "3.0 / 2.0 ")]
    blackjack_payout: f32,

    /// The payout ratio of insurance.
    #[builder(default = "2.0 / 1.0 ")]
    insurance_payout: f32,

    /// Does the dealer hit on soft 17
    #[builder(default = "false")]
    h17: bool,

    /// Is double-after-split allowed?
    #[builder(default = "true")]
    das: bool,

    /// When can the player double down
    #[builder(default = "DoublingDownRule::DoubleOnHard9To11")]
    doubling_down: DoublingDownRule,

    /// When can the player surrender
    #[builder(default = "SurrenderRule::EarlySurrenderAgainst10")]
    surrendering: SurrenderRule,

    /// When does the dealer peek for blackjack
    #[builder(default = "DealerPeekingRule::ENHC")]
    peeking_for_blackjack: DealerPeekingRule,

    /// The maximum number of a times a player is allowed to split their hand.
    #[builder(default = "3")]
    max_splits: u8,

    /// Can the player resplit Aces?
    #[builder(default = "false")]
    can_resplit_aces: bool,

    /// Can the player play the hands from two split Aces?
    #[builder(default = "false")]
    can_play_split_aces: bool,
}

impl TableRules {
    pub fn builder() -> TableRulesBuilder {
        TableRulesBuilder::default()
    }

    pub fn number_of_decks(&self) -> u8 {
        self.number_of_decks
    }
}

#[derive(Debug, Clone, Copy)]
enum DoublingDownRule {
    /// Doubling down is allowed on any two player cards.
    DoubleOnAny2,

    /// Doubling down is allowed only on totals between 9 and 11 (inclusive)
    DoubleOnHard9To11,

    /// Doubling down is allowed only on a hard total of 10 and 11.
    DoubleOnHard10And11,

    /// Doubling down is allowed only on hard or soft totals between 9 and 11 (inclusive).
    DoubleOnAny9To11,
}

#[derive(Debug, Clone, Copy)]
enum SurrenderRule {
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

#[derive(Debug, Clone, Copy)]
enum DealerPeekingRule {
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