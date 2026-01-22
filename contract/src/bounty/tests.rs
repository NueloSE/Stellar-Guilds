use crate::{StellarGuildsContract, StellarGuildsContractClient};
use crate::bounty::types::BountyStatus;
use soroban_sdk::{
    testutils::{Address as _, Ledger},
    Address, Env, String,
};

fn setup() -> (Env, Address, Address, Address, Address, Address) {
    let env = Env::default();
    env.budget().reset_unlimited();
    
    let owner = Address::random(&env);
    let admin = Address::random(&env);
    let member = Address::random(&env);
    let claimer = Address::random(&env);
    let funder = Address::random(&env);
    
    (env, owner, admin, member, claimer, funder)
}

fn register_and_init_contract(env: &Env) -> (Address, StellarGuildsContractClient) {
    let contract_id = env.register_contract(None, StellarGuildsContract);
    let client = StellarGuildsContractClient::new(env, &contract_id);
    client.initialize();
    (contract_id, client)
}

fn create_token(env: &Env, admin: &Address) -> (Address, soroban_sdk::token::Client<'static>) {
    let token_id = env.register_stellar_asset_contract(admin.clone());
    let token = soroban_sdk::token::Client::new(env, &token_id);
    (token_id, token)
}

#[test]
fn test_bounty_lifecycle_success() {
    let (env, owner, _, _, claimer, funder) = setup();
    let (contract_id, client) = register_and_init_contract(&env);
    let (token_id, token_client) = create_token(&env, &funder); // Funder is issuer for simplicity

    // 1. Create Guild
    owner.mock_all_auths();
    let guild_name = String::from_str(&env, "Dev Guild");
    let guild_desc = String::from_str(&env, "Developers");
    let guild_id = client.create_guild(&guild_name, &guild_desc, &owner);

    // 2. Fund the funder account
    token_client.mint(&funder, &1000);

    // 3. Create Bounty
    let title = String::from_str(&env, "Fix Bug");
    let desc = String::from_str(&env, "Fix specific bug");
    let reward = 500i128;
    let expiry = env.ledger().timestamp() + 86400; // +1 day

    let bounty_id = client.create_bounty(
        &guild_id, 
        &owner, 
        &title, 
        &desc, 
        &reward, 
        &token_id, 
        &expiry
    );

    let bounty = client.get_bounty(&bounty_id);
    assert_eq!(bounty.status, BountyStatus::AwaitingFunds);
    assert_eq!(bounty.reward_amount, reward);

    // 4. Fund Bounty
    funder.mock_all_auths();
    // Approve contract to spend tokens (standard flow unless direct transfer implemented in contract, 
    // but here we used `transfer_from`?? No, `lock_funds` uses `client.transfer(funder, contract)`).
    // The `transfer` method in standard token interface sends FROM caller if authed.
    
    let result = client.fund_bounty(&bounty_id, &funder, &reward);
    assert!(result);

    let bounty = client.get_bounty(&bounty_id);
    assert_eq!(bounty.status, BountyStatus::Open);
    assert_eq!(bounty.funded_amount, reward);

    // check contract balance
    assert_eq!(token_client.balance(&contract_id), reward);

    // 5. Claim Bounty
    claimer.mock_all_auths();
    client.claim_bounty(&bounty_id, &claimer);

    let bounty = client.get_bounty(&bounty_id);
    assert_eq!(bounty.status, BountyStatus::Claimed);
    assert_eq!(bounty.claimer, Some(claimer.clone()));

    // 6. Submit Work
    let submission = String::from_str(&env, "http://github.com/pr");
    client.submit_work(&bounty_id, &submission);
    
    let bounty = client.get_bounty(&bounty_id);
    assert_eq!(bounty.status, BountyStatus::UnderReview);

    // 7. Approve Completion
    owner.mock_all_auths();
    client.approve_completion(&bounty_id, &owner);
    
    let bounty = client.get_bounty(&bounty_id);
    assert_eq!(bounty.status, BountyStatus::Completed);

    // 8. Release Funds
    let old_balance = token_client.balance(&claimer);
    
    // Anyone can trigger release if completed, or just owner
    client.release_escrow(&bounty_id);
    
    let new_balance = token_client.balance(&claimer);
    assert_eq!(new_balance, old_balance + reward);
    assert_eq!(token_client.balance(&contract_id), 0);
}

#[test]
fn test_cancel_bounty_refund() {
    let (env, owner, _, _, _, funder) = setup();
    let (contract_id, client) = register_and_init_contract(&env);
    let (token_id, token_client) = create_token(&env, &funder);

    // Create Guild & Bounty
    owner.mock_all_auths();
    let guild_id = client.create_guild(&String::from_str(&env, "G"), &String::from_str(&env, "D"), &owner);
    token_client.mint(&funder, &1000); // give funder tokens

    // Funder funds bounty
    let reward = 200i128;
    let bounty_id = client.create_bounty(
        &guild_id, 
        &owner, 
        &String::from_str(&env, "T"), 
        &String::from_str(&env, "D"), 
        &reward, 
        &token_id, 
        &100000
    );

    funder.mock_all_auths();
    client.fund_bounty(&bounty_id, &funder, &reward);
    
    assert_eq!(token_client.balance(&contract_id), reward);

    // Cancel Bounty (Creator cancels)
    owner.mock_all_auths();
    client.cancel_bounty(&bounty_id, &owner);

    let bounty = client.get_bounty(&bounty_id);
    assert_eq!(bounty.status, BountyStatus::Cancelled);
    assert_eq!(bounty.funded_amount, 0);

    // Check refund (refunds to creator in current impl, even if funder was different - documented limitation)
    // Wait, in my test funder != creator. So funds go to creator (owner).
    assert_eq!(token_client.balance(&owner), reward);
    assert_eq!(token_client.balance(&contract_id), 0);
}

#[test]
#[should_panic(expected = "Bounty expired")]
fn test_claim_expired_bounty() {
    let (env, owner, _, _, claimer, _) = setup();
    let (contract_id, client) = register_and_init_contract(&env);
    let (token_id, _) = create_token(&env, &owner);

    owner.mock_all_auths();
    let guild_id = client.create_guild(&String::from_str(&env, "G"), &String::from_str(&env, "D"), &owner);

    // Create bounty that expires immediately
    let created_at = env.ledger().timestamp();
    let bounty_id = client.create_bounty(
        &guild_id, 
        &owner, 
        &String::from_str(&env, "T"), 
        &String::from_str(&env, "D"), 
        &0, // Open immediately
        &token_id, 
        &created_at // expires now
    );

    // Advance time past expiry
    env.ledger().set_timestamp(created_at + 100);

    claimer.mock_all_auths();
    client.claim_bounty(&bounty_id, &claimer);
}
