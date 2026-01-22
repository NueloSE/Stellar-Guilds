use crate::bounty::escrow::{lock_funds, release_funds};
use crate::bounty::storage::{
    get_bounty, get_guild_bounties, get_next_bounty_id, store_bounty,
};
use crate::bounty::types::{Bounty, BountyStatus};
use crate::guild::membership::has_permission;
use crate::guild::types::Role;
use soroban_sdk::{Address, Env, String, Vec};

pub fn create_bounty(
    env: &Env,
    guild_id: u64,
    creator: Address,
    title: String,
    description: String,
    reward_amount: i128,
    token: Address,
    expiry: u64,
) -> u64 {
    creator.require_auth();

    // Verify creator has Admin or Owner permissions in the guild
    if !has_permission(env, guild_id, creator.clone(), Role::Admin) {
        panic!("Unauthorized: Creator must be a guild admin or owner");
    }

    if reward_amount < 0 {
        panic!("Invalid reward amount");
    }

    let bounty_id = get_next_bounty_id(env);
    let created_at = env.ledger().timestamp();

    let status = if reward_amount == 0 {
        BountyStatus::Open
    } else {
        BountyStatus::AwaitingFunds
    };

    let bounty = Bounty {
        id: bounty_id,
        guild_id,
        creator: creator.clone(),
        title,
        description,
        reward_amount,
        funded_amount: 0,
        token: token.clone(),
        status,
        claimer: None,
        created_at,
        expires_at: expiry, // Absolute timestamp
    };

    store_bounty(env, &bounty);
    
    // TODO: Emit event

    bounty_id
}

pub fn fund_bounty(env: &Env, bounty_id: u64, funder: Address, amount: i128) -> bool {
    funder.require_auth();

    if amount <= 0 {
        panic!("Amount must be positive");
    }

    let mut bounty = get_bounty(env, bounty_id).expect("Bounty not found");

    // Can only fund if awaiting funds or open (partial funding support)
    match bounty.status {
        BountyStatus::AwaitingFunds | BountyStatus::Open => {},
        _ => panic!("Bounty cannot be funded in current status"),
    }

    // Transfer tokens to contract
    lock_funds(env, &bounty.token, &funder, amount);

    // Update state
    bounty.funded_amount += amount;

    if bounty.funded_amount >= bounty.reward_amount && bounty.status == BountyStatus::AwaitingFunds {
        bounty.status = BountyStatus::Open;
    }

    store_bounty(env, &bounty);

    true
}

pub fn claim_bounty(env: &Env, bounty_id: u64, claimer: Address) -> bool {
    claimer.require_auth();

    let mut bounty = get_bounty(env, bounty_id).expect("Bounty not found");

    // Check expiration
    if env.ledger().timestamp() > bounty.expires_at {
        bounty.status = BountyStatus::Expired;
        store_bounty(env, &bounty);
        panic!("Bounty expired");
    }

    if bounty.status != BountyStatus::Open {
        panic!("Bounty not open for claiming");
    }

    bounty.status = BountyStatus::Claimed;
    bounty.claimer = Some(claimer);

    store_bounty(env, &bounty);

    true
}

pub fn submit_work(env: &Env, bounty_id: u64, submission_url: String) -> bool {
    // We need to fetch bounty to check claimer
    let mut bounty = get_bounty(env, bounty_id).expect("Bounty not found");

    if let Some(ref claimer) = bounty.claimer {
        claimer.require_auth();
    } else {
        panic!("No claimer for this bounty");
    }

    if bounty.status != BountyStatus::Claimed {
        panic!("Bounty not in claimed status");
    }

    bounty.status = BountyStatus::UnderReview;
    store_bounty(env, &bounty);

    // Provide visibility of submission via events in a real app
    // env.events().publish((symbol_short!("submit"), bounty_id), submission_url);
    
    true
}

pub fn approve_completion(env: &Env, bounty_id: u64, approver: Address) -> bool {
    approver.require_auth();

    let mut bounty = get_bounty(env, bounty_id).expect("Bounty not found");

    // Verify approver permissions
    if !has_permission(env, bounty.guild_id, approver, Role::Admin) {
        panic!("Unauthorized: Approver must be a guild admin");
    }

    if bounty.status != BountyStatus::UnderReview {
        panic!("Bounty not under review");
    }

    bounty.status = BountyStatus::Completed;
    store_bounty(env, &bounty);

    true
}

pub fn release_escrow(env: &Env, bounty_id: u64) -> bool {
    let bounty = get_bounty(env, bounty_id).expect("Bounty not found");

    if bounty.status != BountyStatus::Completed {
        panic!("Bounty not completed");
    }

    if let Some(claimer) = bounty.claimer {
        if bounty.funded_amount > 0 {
            release_funds(env, &bounty.token, &claimer, bounty.funded_amount);
        }
        true
    } else {
        false
    }
}

pub fn cancel_bounty(env: &Env, bounty_id: u64) -> bool {
    let caller = env.current_contract_address(); // Check context, usually cancellation needs auth
    // But here we need to know WHO is cancelling. 'env' doesn't give us 'caller' directly in Soroban 
    // without passed argument. The 'cancel_bounty' core function signature in prompt is:
    // cancel_bounty(env, bounty_id) -> bool
    // This implies the caller is implied or we need to pass it? 
    // Actually, usually we pass 'caller: Address' to verify auth. 
    // The prompt signature is "cancel_bounty(env, bounty_id) -> bool". 
    // This assumes we can't check auth unless we peek at call stack or require invoker... 
    // But idiomatic Soroban usually passes the address.
    // I will modify the signature to include `canceller: Address` for security.
    panic!("Missing canceller argument for auth check");
}

// Fixed signature with canceller
pub fn cancel_bounty_auth(env: &Env, bounty_id: u64, canceller: Address) -> bool {
    canceller.require_auth();

    let mut bounty = get_bounty(env, bounty_id).expect("Bounty not found");

    // Creator or Admin can cancel
    let is_creator = bounty.creator == canceller;
    let is_admin = has_permission(env, bounty.guild_id, canceller.clone(), Role::Admin);

    if !is_creator && !is_admin {
        panic!("Unauthorized to cancel bounty");
    }

    // Refund logic
    if bounty.funded_amount > 0 {
        // Refund to creator for now (since we don't track original funder separately yet)
        release_funds(env, &bounty.token, &bounty.creator, bounty.funded_amount);
        bounty.funded_amount = 0;
    }

    bounty.status = BountyStatus::Cancelled;
    store_bounty(env, &bounty);

    true
}

// Queries
pub fn get_bounty_data(env: &Env, bounty_id: u64) -> Bounty {
    get_bounty(env, bounty_id).expect("Bounty not found")
}

pub fn get_guild_bounties_list(env: &Env, guild_id: u64) -> Vec<Bounty> {
    get_guild_bounties(env, guild_id)
}

#[cfg(test)]
mod tests;
