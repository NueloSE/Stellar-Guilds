/// Payment distribution module
///
/// This module provides automated payment splitting functionality for the Stellar Guilds platform.
/// It enables fair distribution of rewards among multiple contributors based on configurable rules.
///
/// # Overview
/// - `types`: Defines all core data structures and events
/// - `storage`: Manages persistent storage of payment pools and recipients
/// - `distribution`: Core functions for payment pool management and distribution
///
/// # Distribution Rules
/// - **Percentage**: Recipients get fixed percentage shares (must sum to 100%)
/// - **EqualSplit**: All recipients get equal shares
/// - **Weighted**: Recipients get shares proportional to their weights
///
/// # Key Features
/// - Atomic distribution execution
/// - Support for native XLM and custom tokens
/// - Dust amount protection
/// - Comprehensive validation
/// - Event emission for transparency

pub mod types;
pub mod storage;
pub mod distribution;

// Re-export main functions for convenience
pub use distribution::{
    create_payment_pool, add_recipient, validate_distribution, execute_distribution,
    get_recipient_amount, cancel_distribution, get_pool_status, batch_distribute,
};
pub use types::{PaymentPool, Recipient, DistributionRule, DistributionStatus};
pub use storage::{initialize_payment_storage};