/// Guild management module
/// 
/// This module provides all guild creation, membership management, and role-based
/// permission functionality for the Stellar Guilds platform.
///
/// # Overview
/// - `types`: Defines all core data structures (Guild, Member, Role, Events)
/// - `storage`: Manages persistent storage of guilds and members
/// - `membership`: Core functions for guild and member management

pub mod types;
pub mod storage;
pub mod membership;

pub use membership::{
    create_guild, add_member, remove_member, update_role,
    get_member, get_all_members, is_member, has_permission,
};
pub use types::{Guild, Member, Role};
