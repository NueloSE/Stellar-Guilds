#![no_std]

use soroban_sdk::{contract, contractimpl, Address, Env, String, Symbol, Vec};

mod guild;

use guild::membership::{
    self, create_guild, add_member, remove_member, update_role, get_member,
    get_all_members, is_member, has_permission,
};
use guild::storage;
use guild::types::{Guild, Member, Role};

/// Stellar Guilds - Main Contract Entry Point
/// 
/// This is the foundational contract for the Stellar Guilds platform.
/// It enables users to create guilds, add members, assign roles, and manage
/// permissions on-chain. This forms the foundation for decentralized communities,
/// voting, and role-based governance.
///
/// # Core Features
/// - Guild creation with metadata
/// - Member management with role assignments
/// - Permission-based access control
/// - Event tracking for all state changes
/// - Efficient on-chain storage management

#[contract]
pub struct StellarGuildsContract;

#[contractimpl]
impl StellarGuildsContract {
    /// Initialize the contract
    /// 
    /// Sets up the storage structures for guilds and members.
    /// Must be called before any other operations.
    pub fn initialize(env: Env) -> bool {
        storage::initialize(&env);
        true
    }

    /// Get contract version
    pub fn version(_env: Env) -> String {
        String::from_slice(&_env, "0.1.0")
    }

    /// Create a new guild
    ///
    /// # Arguments
    /// * `name` - The name of the guild
    /// * `description` - The description of the guild
    /// * `owner` - The address of the guild owner
    ///
    /// # Returns
    /// The ID of the newly created guild
    pub fn create_guild(
        env: Env,
        name: String,
        description: String,
        owner: Address,
    ) -> Result<u64, String> {
        owner.require_auth();
        create_guild(&env, name, description, owner)
    }

    /// Add a member to a guild
    ///
    /// # Arguments
    /// * `guild_id` - The ID of the guild
    /// * `address` - The address of the member to add
    /// * `role` - The role to assign
    /// * `caller` - The address making the request (must have permission)
    ///
    /// # Returns
    /// true if successful, error string otherwise
    pub fn add_member(
        env: Env,
        guild_id: u64,
        address: Address,
        role: Role,
        caller: Address,
    ) -> Result<bool, String> {
        caller.require_auth();
        add_member(&env, guild_id, address, role, caller)
    }

    /// Remove a member from a guild
    ///
    /// # Arguments
    /// * `guild_id` - The ID of the guild
    /// * `address` - The address of the member to remove
    /// * `caller` - The address making the request
    ///
    /// # Returns
    /// true if successful, error string otherwise
    pub fn remove_member(
        env: Env,
        guild_id: u64,
        address: Address,
        caller: Address,
    ) -> Result<bool, String> {
        caller.require_auth();
        remove_member(&env, guild_id, address, caller)
    }

    /// Update a member's role
    ///
    /// # Arguments
    /// * `guild_id` - The ID of the guild
    /// * `address` - The address of the member
    /// * `new_role` - The new role to assign
    /// * `caller` - The address making the request (must have permission)
    ///
    /// # Returns
    /// true if successful, error string otherwise
    pub fn update_role(
        env: Env,
        guild_id: u64,
        address: Address,
        new_role: Role,
        caller: Address,
    ) -> Result<bool, String> {
        caller.require_auth();
        update_role(&env, guild_id, address, new_role, caller)
    }

    /// Get a member from a guild
    ///
    /// # Arguments
    /// * `guild_id` - The ID of the guild
    /// * `address` - The address of the member
    ///
    /// # Returns
    /// The Member if found, error string otherwise
    pub fn get_member(env: Env, guild_id: u64, address: Address) -> Result<Member, String> {
        get_member(&env, guild_id, address)
    }

    /// Get all members of a guild
    ///
    /// # Arguments
    /// * `guild_id` - The ID of the guild
    ///
    /// # Returns
    /// A vector of all members in the guild
    pub fn get_all_members(env: Env, guild_id: u64) -> Vec<Member> {
        get_all_members(&env, guild_id)
    }

    /// Check if an address is a member of a guild
    ///
    /// # Arguments
    /// * `guild_id` - The ID of the guild
    /// * `address` - The address to check
    ///
    /// # Returns
    /// true if the address is a member, false otherwise
    pub fn is_member(env: Env, guild_id: u64, address: Address) -> bool {
        is_member(&env, guild_id, address)
    }

    /// Check if a member has permission for a required role
    ///
    /// # Arguments
    /// * `guild_id` - The ID of the guild
    /// * `address` - The address of the member
    /// * `required_role` - The required role level
    ///
    /// # Returns
    /// true if the member has the required permission, false otherwise
    pub fn has_permission(
        env: Env,
        guild_id: u64,
        address: Address,
        required_role: Role,
    ) -> bool {
        has_permission(&env, guild_id, address, required_role)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::Env;

    fn setup() -> (Env, Address, Address, Address, Address) {
        let env = Env::default();
        env.budget().reset_unlimited();
        
        let owner = Address::random(&env);
        let admin = Address::random(&env);
        let member = Address::random(&env);
        let non_member = Address::random(&env);
        
        (env, owner, admin, member, non_member)
    }

    fn register_and_init_contract(env: &Env) -> Address {
        let contract_id = env.register_contract(None, StellarGuildsContract);
        let client = StellarGuildsContractClient::new(env, &contract_id);
        
        client.initialize();
        
        contract_id
    }

    // ============ Initialization Tests ============

    #[test]
    fn test_initialize() {
        let (env, _, _, _, _) = setup();
        let contract_id = register_and_init_contract(&env);
        
        // Verify initialization was successful
        let client = StellarGuildsContractClient::new(&env, &contract_id);
        let result = client.initialize();
        assert_eq!(result, true);
    }

    #[test]
    fn test_version() {
        let (env, _, _, _, _) = setup();
        let contract_id = register_and_init_contract(&env);
        
        let client = StellarGuildsContractClient::new(&env, &contract_id);
        let version = client.version();
        assert_eq!(version, String::from_slice(&env, "0.1.0"));
    }

    // ============ Guild Creation Tests ============

    #[test]
    fn test_create_guild_success() {
        let (env, owner, _, _, _) = setup();
        let contract_id = register_and_init_contract(&env);
        let client = StellarGuildsContractClient::new(&env, &contract_id);
        
        owner.mock_all_auths();
        
        let name = String::from_slice(&env, "Test Guild");
        let description = String::from_slice(&env, "A test guild");
        
        let result = client.create_guild(&name, &description, &owner);
        assert!(result.is_ok());
        
        let guild_id = result.unwrap();
        assert_eq!(guild_id, 1u64);
    }

    #[test]
    fn test_create_guild_owner_is_member() {
        let (env, owner, _, _, _) = setup();
        let contract_id = register_and_init_contract(&env);
        let client = StellarGuildsContractClient::new(&env, &contract_id);
        
        owner.mock_all_auths();
        
        let name = String::from_slice(&env, "Guild");
        let description = String::from_slice(&env, "Description");
        
        let guild_id = client.create_guild(&name, &description, &owner).unwrap();
        
        // Owner should be a member after creation
        let is_member = client.is_member(&guild_id, &owner);
        assert_eq!(is_member, true);
        
        let member = client.get_member(&guild_id, &owner).unwrap();
        assert_eq!(member.role, Role::Owner);
    }

    #[test]
    fn test_create_guild_invalid_name_empty() {
        let (env, owner, _, _, _) = setup();
        let contract_id = register_and_init_contract(&env);
        let client = StellarGuildsContractClient::new(&env, &contract_id);
        
        owner.mock_all_auths();
        
        let name = String::from_slice(&env, "");
        let description = String::from_slice(&env, "Description");
        
        let result = client.create_guild(&name, &description, &owner);
        assert!(result.is_err());
    }

    #[test]
    fn test_create_guild_invalid_description_too_long() {
        let (env, owner, _, _, _) = setup();
        let contract_id = register_and_init_contract(&env);
        let client = StellarGuildsContractClient::new(&env, &contract_id);
        
        owner.mock_all_auths();
        
        let name = String::from_slice(&env, "Guild");
        // Create a description that is too long (> 512 chars)
        let long_desc = "x".repeat(513);
        let description = String::from_slice(&env, &long_desc);
        
        let result = client.create_guild(&name, &description, &owner);
        assert!(result.is_err());
    }

    #[test]
    fn test_create_multiple_guilds() {
        let (env, owner, _, _, _) = setup();
        let contract_id = register_and_init_contract(&env);
        let client = StellarGuildsContractClient::new(&env, &contract_id);
        
        owner.mock_all_auths();
        
        let name1 = String::from_slice(&env, "Guild 1");
        let description1 = String::from_slice(&env, "First guild");
        
        let guild_id_1 = client.create_guild(&name1, &description1, &owner).unwrap();
        
        let name2 = String::from_slice(&env, "Guild 2");
        let description2 = String::from_slice(&env, "Second guild");
        
        let guild_id_2 = client.create_guild(&name2, &description2, &owner).unwrap();
        
        // Guild IDs should be unique and incremental
        assert_eq!(guild_id_1, 1u64);
        assert_eq!(guild_id_2, 2u64);
    }

    // ============ Member Addition Tests ============

    #[test]
    fn test_add_member_by_owner() {
        let (env, owner, admin, _, _) = setup();
        let contract_id = register_and_init_contract(&env);
        let client = StellarGuildsContractClient::new(&env, &contract_id);
        
        owner.mock_all_auths();
        admin.mock_all_auths();
        
        let name = String::from_slice(&env, "Guild");
        let description = String::from_slice(&env, "Description");
        
        let guild_id = client.create_guild(&name, &description, &owner).unwrap();
        
        // Owner adds admin
        let result = client.add_member(&guild_id, &admin, &Role::Admin, &owner);
        assert!(result.is_ok());
        
        let member = client.get_member(&guild_id, &admin).unwrap();
        assert_eq!(member.role, Role::Admin);
    }

    #[test]
    fn test_add_member_duplicate() {
        let (env, owner, admin, _, _) = setup();
        let contract_id = register_and_init_contract(&env);
        let client = StellarGuildsContractClient::new(&env, &contract_id);
        
        owner.mock_all_auths();
        admin.mock_all_auths();
        
        let name = String::from_slice(&env, "Guild");
        let description = String::from_slice(&env, "Description");
        
        let guild_id = client.create_guild(&name, &description, &owner).unwrap();
        
        // Add member once
        client.add_member(&guild_id, &admin, &Role::Member, &owner).unwrap();
        
        // Try to add same member again - should fail
        let result = client.add_member(&guild_id, &admin, &Role::Member, &owner);
        assert!(result.is_err());
    }

    #[test]
    fn test_add_member_permission_denied() {
        let (env, owner, admin, member, non_member) = setup();
        let contract_id = register_and_init_contract(&env);
        let client = StellarGuildsContractClient::new(&env, &contract_id);
        
        owner.mock_all_auths();
        admin.mock_all_auths();
        member.mock_all_auths();
        non_member.mock_all_auths();
        
        let name = String::from_slice(&env, "Guild");
        let description = String::from_slice(&env, "Description");
        
        let guild_id = client.create_guild(&name, &description, &owner).unwrap();
        
        // Add admin
        client.add_member(&guild_id, &admin, &Role::Admin, &owner).unwrap();
        
        // Add member
        client.add_member(&guild_id, &member, &Role::Member, &owner).unwrap();
        
        // Non-member tries to add someone - should fail
        let result = client.add_member(&guild_id, &non_member, &Role::Member, &non_member);
        assert!(result.is_err());
        
        // Member tries to add admin - should fail
        let another = Address::random(&env);
        another.mock_all_auths();
        let result = client.add_member(&guild_id, &another, &Role::Admin, &member);
        assert!(result.is_err());
    }

    #[test]
    fn test_add_admin_by_non_owner() {
        let (env, owner, admin, member, _) = setup();
        let contract_id = register_and_init_contract(&env);
        let client = StellarGuildsContractClient::new(&env, &contract_id);
        
        owner.mock_all_auths();
        admin.mock_all_auths();
        member.mock_all_auths();
        
        let name = String::from_slice(&env, "Guild");
        let description = String::from_slice(&env, "Description");
        
        let guild_id = client.create_guild(&name, &description, &owner).unwrap();
        
        // Add member
        client.add_member(&guild_id, &member, &Role::Member, &owner).unwrap();
        
        // Member tries to add an owner - should fail
        let new_owner = Address::random(&env);
        new_owner.mock_all_auths();
        
        let result = client.add_member(&guild_id, &new_owner, &Role::Owner, &member);
        assert!(result.is_err());
    }

    // ============ Member Removal Tests ============

    #[test]
    fn test_remove_member_by_owner() {
        let (env, owner, member, _, _) = setup();
        let contract_id = register_and_init_contract(&env);
        let client = StellarGuildsContractClient::new(&env, &contract_id);
        
        owner.mock_all_auths();
        member.mock_all_auths();
        
        let name = String::from_slice(&env, "Guild");
        let description = String::from_slice(&env, "Description");
        
        let guild_id = client.create_guild(&name, &description, &owner).unwrap();
        
        // Add member
        client.add_member(&guild_id, &member, &Role::Member, &owner).unwrap();
        
        // Verify member exists
        let is_member = client.is_member(&guild_id, &member);
        assert_eq!(is_member, true);
        
        // Remove member
        let result = client.remove_member(&guild_id, &member, &owner);
        assert!(result.is_ok());
        
        // Verify member no longer exists
        let is_member = client.is_member(&guild_id, &member);
        assert_eq!(is_member, false);
    }

    #[test]
    fn test_self_removal() {
        let (env, owner, member, _, _) = setup();
        let contract_id = register_and_init_contract(&env);
        let client = StellarGuildsContractClient::new(&env, &contract_id);
        
        owner.mock_all_auths();
        member.mock_all_auths();
        
        let name = String::from_slice(&env, "Guild");
        let description = String::from_slice(&env, "Description");
        
        let guild_id = client.create_guild(&name, &description, &owner).unwrap();
        
        // Add member
        client.add_member(&guild_id, &member, &Role::Member, &owner).unwrap();
        
        // Member removes themselves
        let result = client.remove_member(&guild_id, &member, &member);
        assert!(result.is_ok());
        
        // Verify member no longer exists
        let is_member = client.is_member(&guild_id, &member);
        assert_eq!(is_member, false);
    }

    #[test]
    fn test_remove_last_owner_fails() {
        let (env, owner, _, _, _) = setup();
        let contract_id = register_and_init_contract(&env);
        let client = StellarGuildsContractClient::new(&env, &contract_id);
        
        owner.mock_all_auths();
        
        let name = String::from_slice(&env, "Guild");
        let description = String::from_slice(&env, "Description");
        
        let guild_id = client.create_guild(&name, &description, &owner).unwrap();
        
        // Try to remove the only owner - should fail
        let result = client.remove_member(&guild_id, &owner, &owner);
        assert!(result.is_err());
    }

    #[test]
    fn test_remove_non_owner_by_non_owner_fails() {
        let (env, owner, admin, member, _) = setup();
        let contract_id = register_and_init_contract(&env);
        let client = StellarGuildsContractClient::new(&env, &contract_id);
        
        owner.mock_all_auths();
        admin.mock_all_auths();
        member.mock_all_auths();
        
        let name = String::from_slice(&env, "Guild");
        let description = String::from_slice(&env, "Description");
        
        let guild_id = client.create_guild(&name, &description, &owner).unwrap();
        
        // Add member and admin
        client.add_member(&guild_id, &member, &Role::Member, &owner).unwrap();
        client.add_member(&guild_id, &admin, &Role::Admin, &owner).unwrap();
        
        // Member tries to remove admin - should fail
        let result = client.remove_member(&guild_id, &admin, &member);
        assert!(result.is_err());
    }

    // ============ Role Update Tests ============

    #[test]
    fn test_update_role_by_owner() {
        let (env, owner, member, _, _) = setup();
        let contract_id = register_and_init_contract(&env);
        let client = StellarGuildsContractClient::new(&env, &contract_id);
        
        owner.mock_all_auths();
        member.mock_all_auths();
        
        let name = String::from_slice(&env, "Guild");
        let description = String::from_slice(&env, "Description");
        
        let guild_id = client.create_guild(&name, &description, &owner).unwrap();
        
        // Add member
        client.add_member(&guild_id, &member, &Role::Member, &owner).unwrap();
        
        // Update to admin
        let result = client.update_role(&guild_id, &member, &Role::Admin, &owner);
        assert!(result.is_ok());
        
        let updated_member = client.get_member(&guild_id, &member).unwrap();
        assert_eq!(updated_member.role, Role::Admin);
    }

    #[test]
    fn test_update_role_permission_denied() {
        let (env, owner, member1, member2, _) = setup();
        let contract_id = register_and_init_contract(&env);
        let client = StellarGuildsContractClient::new(&env, &contract_id);
        
        owner.mock_all_auths();
        member1.mock_all_auths();
        member2.mock_all_auths();
        
        let name = String::from_slice(&env, "Guild");
        let description = String::from_slice(&env, "Description");
        
        let guild_id = client.create_guild(&name, &description, &owner).unwrap();
        
        // Add members
        client.add_member(&guild_id, &member1, &Role::Member, &owner).unwrap();
        client.add_member(&guild_id, &member2, &Role::Member, &owner).unwrap();
        
        // Member1 tries to change member2's role - should fail
        let result = client.update_role(&guild_id, &member2, &Role::Admin, &member1);
        assert!(result.is_err());
    }

    #[test]
    fn test_cannot_demote_last_owner() {
        let (env, owner, admin, _, _) = setup();
        let contract_id = register_and_init_contract(&env);
        let client = StellarGuildsContractClient::new(&env, &contract_id);
        
        owner.mock_all_auths();
        admin.mock_all_auths();
        
        let name = String::from_slice(&env, "Guild");
        let description = String::from_slice(&env, "Description");
        
        let guild_id = client.create_guild(&name, &description, &owner).unwrap();
        
        // Add admin
        client.add_member(&guild_id, &admin, &Role::Admin, &owner).unwrap();
        
        // Try to demote the last owner - should fail
        let result = client.update_role(&guild_id, &owner, &Role::Admin, &owner);
        assert!(result.is_err());
    }

    #[test]
    fn test_can_demote_owner_if_multiple() {
        let (env, owner1, owner2, member, _) = setup();
        let contract_id = register_and_init_contract(&env);
        let client = StellarGuildsContractClient::new(&env, &contract_id);
        
        owner1.mock_all_auths();
        owner2.mock_all_auths();
        member.mock_all_auths();
        
        let name = String::from_slice(&env, "Guild");
        let description = String::from_slice(&env, "Description");
        
        let guild_id = client.create_guild(&name, &description, &owner1).unwrap();
        
        // Add owner2
        client.add_member(&guild_id, &owner2, &Role::Owner, &owner1).unwrap();
        
        // Now owner1 can be demoted
        let result = client.update_role(&guild_id, &owner1, &Role::Admin, &owner1);
        assert!(result.is_ok());
    }

    // ============ Member Query Tests ============

    #[test]
    fn test_get_member() {
        let (env, owner, member, _, _) = setup();
        let contract_id = register_and_init_contract(&env);
        let client = StellarGuildsContractClient::new(&env, &contract_id);
        
        owner.mock_all_auths();
        member.mock_all_auths();
        
        let name = String::from_slice(&env, "Guild");
        let description = String::from_slice(&env, "Description");
        
        let guild_id = client.create_guild(&name, &description, &owner).unwrap();
        
        client.add_member(&guild_id, &member, &Role::Member, &owner).unwrap();
        
        let result = client.get_member(&guild_id, &member);
        assert!(result.is_ok());
        
        let member_data = result.unwrap();
        assert_eq!(member_data.address, member);
        assert_eq!(member_data.role, Role::Member);
    }

    #[test]
    fn test_get_member_not_found() {
        let (env, owner, member, non_member, _) = setup();
        let contract_id = register_and_init_contract(&env);
        let client = StellarGuildsContractClient::new(&env, &contract_id);
        
        owner.mock_all_auths();
        member.mock_all_auths();
        
        let name = String::from_slice(&env, "Guild");
        let description = String::from_slice(&env, "Description");
        
        let guild_id = client.create_guild(&name, &description, &owner).unwrap();
        
        client.add_member(&guild_id, &member, &Role::Member, &owner).unwrap();
        
        let result = client.get_member(&guild_id, &non_member);
        assert!(result.is_err());
    }

    #[test]
    fn test_get_all_members() {
        let (env, owner, member1, member2, member3) = setup();
        let contract_id = register_and_init_contract(&env);
        let client = StellarGuildsContractClient::new(&env, &contract_id);
        
        owner.mock_all_auths();
        member1.mock_all_auths();
        member2.mock_all_auths();
        member3.mock_all_auths();
        
        let name = String::from_slice(&env, "Guild");
        let description = String::from_slice(&env, "Description");
        
        let guild_id = client.create_guild(&name, &description, &owner).unwrap();
        
        // Initially should have 1 member (owner)
        let mut members = client.get_all_members(&guild_id);
        assert_eq!(members.len(), 1);
        
        // Add more members
        client.add_member(&guild_id, &member1, &Role::Member, &owner).unwrap();
        client.add_member(&guild_id, &member2, &Role::Admin, &owner).unwrap();
        client.add_member(&guild_id, &member3, &Role::Contributor, &owner).unwrap();
        
        // Should now have 4 members
        members = client.get_all_members(&guild_id);
        assert_eq!(members.len(), 4);
    }

    #[test]
    fn test_is_member() {
        let (env, owner, member, non_member, _) = setup();
        let contract_id = register_and_init_contract(&env);
        let client = StellarGuildsContractClient::new(&env, &contract_id);
        
        owner.mock_all_auths();
        member.mock_all_auths();
        
        let name = String::from_slice(&env, "Guild");
        let description = String::from_slice(&env, "Description");
        
        let guild_id = client.create_guild(&name, &description, &owner).unwrap();
        
        // Owner should be a member
        assert_eq!(client.is_member(&guild_id, &owner), true);
        
        // Non-member should not be a member
        assert_eq!(client.is_member(&guild_id, &non_member), false);
        
        // Add member
        client.add_member(&guild_id, &member, &Role::Member, &owner).unwrap();
        
        // Member should now be a member
        assert_eq!(client.is_member(&guild_id, &member), true);
    }

    // ============ Permission Tests ============

    #[test]
    fn test_has_permission() {
        let (env, owner, admin, member, contributor) = setup();
        let contract_id = register_and_init_contract(&env);
        let client = StellarGuildsContractClient::new(&env, &contract_id);
        
        owner.mock_all_auths();
        admin.mock_all_auths();
        member.mock_all_auths();
        contributor.mock_all_auths();
        
        let name = String::from_slice(&env, "Guild");
        let description = String::from_slice(&env, "Description");
        
        let guild_id = client.create_guild(&name, &description, &owner).unwrap();
        
        client.add_member(&guild_id, &admin, &Role::Admin, &owner).unwrap();
        client.add_member(&guild_id, &member, &Role::Member, &owner).unwrap();
        client.add_member(&guild_id, &contributor, &Role::Contributor, &owner).unwrap();
        
        // Owner has all permissions
        assert_eq!(client.has_permission(&guild_id, &owner, &Role::Owner), true);
        assert_eq!(client.has_permission(&guild_id, &owner, &Role::Admin), true);
        assert_eq!(client.has_permission(&guild_id, &owner, &Role::Member), true);
        assert_eq!(client.has_permission(&guild_id, &owner, &Role::Contributor), true);
        
        // Admin has admin and below permissions
        assert_eq!(client.has_permission(&guild_id, &admin, &Role::Owner), false);
        assert_eq!(client.has_permission(&guild_id, &admin, &Role::Admin), true);
        assert_eq!(client.has_permission(&guild_id, &admin, &Role::Member), true);
        assert_eq!(client.has_permission(&guild_id, &admin, &Role::Contributor), true);
        
        // Member has member and below permissions
        assert_eq!(client.has_permission(&guild_id, &member, &Role::Owner), false);
        assert_eq!(client.has_permission(&guild_id, &member, &Role::Admin), false);
        assert_eq!(client.has_permission(&guild_id, &member, &Role::Member), true);
        assert_eq!(client.has_permission(&guild_id, &member, &Role::Contributor), true);
        
        // Contributor has only contributor permissions
        assert_eq!(client.has_permission(&guild_id, &contributor, &Role::Owner), false);
        assert_eq!(client.has_permission(&guild_id, &contributor, &Role::Admin), false);
        assert_eq!(client.has_permission(&guild_id, &contributor, &Role::Member), false);
        assert_eq!(client.has_permission(&guild_id, &contributor, &Role::Contributor), true);
    }

    // ============ Guild Lifecycle Integration Tests ============

    #[test]
    fn test_full_guild_lifecycle() {
        let (env, owner, admin, member1, member2) = setup();
        let contract_id = register_and_init_contract(&env);
        let client = StellarGuildsContractClient::new(&env, &contract_id);
        
        owner.mock_all_auths();
        admin.mock_all_auths();
        member1.mock_all_auths();
        member2.mock_all_auths();
        
        // Create guild
        let name = String::from_slice(&env, "Community Guild");
        let description = String::from_slice(&env, "A thriving community");
        
        let guild_id = client.create_guild(&name, &description, &owner).unwrap();
        assert_eq!(guild_id, 1u64);
        
        // Add admin
        client.add_member(&guild_id, &admin, &Role::Admin, &owner).unwrap();
        
        // Add members
        client.add_member(&guild_id, &member1, &Role::Member, &admin).unwrap();
        client.add_member(&guild_id, &member2, &Role::Contributor, &owner).unwrap();
        
        // Verify all members exist
        let members = client.get_all_members(&guild_id);
        assert_eq!(members.len(), 4); // owner + admin + member1 + member2
        
        // Promote member1 to member
        client.update_role(&guild_id, &member1, &Role::Member, &admin).unwrap();
        
        // member1 removes themselves
        client.remove_member(&guild_id, &member1, &member1).unwrap();
        
        // Verify member1 is gone
        let members = client.get_all_members(&guild_id);
        assert_eq!(members.len(), 3);
        
        assert_eq!(client.is_member(&guild_id, &member1), false);
        assert_eq!(client.is_member(&guild_id, &member2), true);
    }

    #[test]
    fn test_admin_can_add_members_and_contributors() {
        let (env, owner, admin, member, contributor) = setup();
        let contract_id = register_and_init_contract(&env);
        let client = StellarGuildsContractClient::new(&env, &contract_id);
        
        owner.mock_all_auths();
        admin.mock_all_auths();
        member.mock_all_auths();
        contributor.mock_all_auths();
        
        let name = String::from_slice(&env, "Guild");
        let description = String::from_slice(&env, "Description");
        
        let guild_id = client.create_guild(&name, &description, &owner).unwrap();
        
        // Add admin
        client.add_member(&guild_id, &admin, &Role::Admin, &owner).unwrap();
        
        // Admin adds member and contributor
        let result1 = client.add_member(&guild_id, &member, &Role::Member, &admin);
        assert!(result1.is_ok());
        
        let result2 = client.add_member(&guild_id, &contributor, &Role::Contributor, &admin);
        assert!(result2.is_ok());
        
        // Verify they were added
        assert_eq!(client.is_member(&guild_id, &member), true);
        assert_eq!(client.is_member(&guild_id, &contributor), true);
    }

    #[test]
    fn test_admin_cannot_add_owner() {
        let (env, owner, admin, new_owner, _) = setup();
        let contract_id = register_and_init_contract(&env);
        let client = StellarGuildsContractClient::new(&env, &contract_id);
        
        owner.mock_all_auths();
        admin.mock_all_auths();
        new_owner.mock_all_auths();
        
        let name = String::from_slice(&env, "Guild");
        let description = String::from_slice(&env, "Description");
        
        let guild_id = client.create_guild(&name, &description, &owner).unwrap();
        
        // Add admin
        client.add_member(&guild_id, &admin, &Role::Admin, &owner).unwrap();
        
        // Admin tries to add owner - should fail
        let result = client.add_member(&guild_id, &new_owner, &Role::Owner, &admin);
        assert!(result.is_err());
    }
}

