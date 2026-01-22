use soroban_sdk::{contracttype, Address, String};

/// Status of a bounty lifecycle
#[contracttype]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BountyStatus {
    Open = 0,
    Claimed = 1,
    UnderReview = 2,
    Completed = 3,
    Cancelled = 4,
    Expired = 5,
    AwaitingFunds = 6,
}

/// Bounty struct containing all bounty metadata and state
#[contracttype]
#[derive(Clone, Debug)]
pub struct Bounty {
    /// Unique identifier for the bounty
    pub id: u64,
    /// ID of the guild this bounty belongs to
    pub guild_id: u64,
    /// Address of the creator (must be guild admin/owner)
    pub creator: Address,
    /// Short title metadata
    pub title: String,
    /// Detailed description
    pub description: String,
    /// Amount of tokens defined as reward
    pub reward_amount: i128,
    /// Amount of tokens currently funded
    pub funded_amount: i128,
    /// Address of the token contract (XLM or custom)
    pub token: Address,
    /// Current status of the bounty
    pub status: BountyStatus,
    /// Address of the contributor who claimed the bounty (optional)
    pub claimer: Option<Address>,
    /// Creation timestamp (seconds)
    pub created_at: u64,
    /// Expiration timestamp (seconds)
    pub expires_at: u64,
}
