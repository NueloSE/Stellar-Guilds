# Stellar Guilds Smart Contracts

This directory contains all Soroban smart contracts for the Stellar Guilds platform.

## Prerequisites

### Windows Setup

1. **Install Visual Studio Build Tools**
   - Download [Visual Studio Build Tools](https://visualstudio.microsoft.com/downloads/)
   - Install with "Desktop development with C++" workload
   - This provides the `link.exe` linker required for Rust compilation

2. **Install Rust**
   ```powershell
   # Download and run rustup-init.exe from https://rustup.rs/
   rustup-init.exe
   ```

3. **Add WASM target**
   ```powershell
   rustup target add wasm32-unknown-unknown
   ```

4. **Install Soroban CLI**
   ```powershell
   cargo install --locked soroban-cli
   ```

5. **Verify Installation**
   ```powershell
   soroban --version
   cargo --version
   rustc --version
   ```

## Project Structure

```
contract/
├── src/
│   ├── lib.rs              # Main contract implementation
│   ├── guild/              # Guild membership module
│   │   ├── mod.rs
│   │   ├── membership.rs
│   │   ├── storage.rs
│   │   └── types.rs
│   └── payment/            # Payment distribution module
│       ├── mod.rs
│       ├── distribution.rs
│       ├── storage.rs
│       └── types.rs
├── Cargo.toml              # Rust project configuration
└── README.md               # This file
```

## Implemented Contract Modules

### Guild Membership Module
- ✅ Guild creation with metadata
- ✅ Member management with role assignments  
- ✅ Permission-based access control
- ✅ Event tracking for all state changes

### Payment Distribution Module
- ✅ Payment pool creation with multiple recipients
- ✅ Percentage-based distribution rules
- ✅ Equal split distributions
- ✅ Weighted distribution based on contribution
- ✅ Automatic payment execution
- ✅ Distribution rule validation
- ✅ Support for XLM and custom tokens
- ✅ Atomic execution (all payments succeed or all fail)
- ✅ Event emission for transparency
- ✅ Comprehensive unit tests

## Planned Contract Modules

The following contract modules are planned for future implementation:

- **Guild Contracts**: Guild creation, membership, and governance
- **Bounty Contracts**: Task creation, escrow, and completion tracking
- **Payment Contracts**: Multi-party payment distribution and splits
- **Milestone Contracts**: Milestone tracking and automated releases
- **Dispute Contracts**: Arbitration and conflict resolution
- **Reputation Contracts**: Contributor reputation and incentive tracking

## Development

### Build Contract

```powershell
# Build for WASM target (required for Soroban)
cargo build --target wasm32-unknown-unknown --release

# The compiled WASM will be in:
# target/wasm32-unknown-unknown/release/stellar_guilds_contract.wasm
```

### Run Tests

```powershell
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test test_initialize
```

### Optimize Contract

```powershell
# Use soroban-cli to optimize the WASM
soroban contract optimize \
  --wasm target/wasm32-unknown-unknown/release/stellar_guilds_contract.wasm
```

## Deployment

### Deploy to Testnet

```powershell
# Set up testnet identity
soroban keys generate --network testnet test-identity

# Fund the test account
soroban keys fund test-identity --network testnet

# Deploy contract
soroban contract deploy \
  --wasm target/wasm32-unknown-unknown/release/stellar_guilds_contract.wasm \
  --source test-identity \
  --network testnet
```

### Deploy to Mainnet

```powershell
# Deploy to mainnet (use with caution!)
soroban contract deploy \
  --wasm target/wasm32-unknown-unknown/release/stellar_guilds_contract.wasm \
  --source YOUR_MAINNET_KEY \
  --network mainnet
```

## Interacting with Contracts

### Invoke Contract Function

```powershell
# Initialize contract
soroban contract invoke \
  --id CONTRACT_ID \
  --source test-identity \
  --network testnet \
  -- initialize

# Get version
soroban contract invoke \
  --id CONTRACT_ID \
  --source test-identity \
  --network testnet \
  -- version
```

## Testing

### Unit Tests

Unit tests are written using the Soroban SDK's test utilities:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::Env;

    #[test]
    fn test_example() {
        let env = Env::default();
        let contract_id = env.register_contract(None, StellarGuildsContract);
        let client = StellarGuildsContractClient::new(&env, &contract_id);
        
        // Test logic here
    }
}
```

### Integration Tests

Integration tests will be added in the `tests/` directory for end-to-end testing.

## Troubleshooting

### Common Issues

1. **"linker `link.exe` not found"**
   - Install Visual Studio Build Tools with C++ development tools
   - Restart your terminal after installation

2. **"target not installed"**
   - Run: `rustup target add wasm32-unknown-unknown`

3. **Soroban CLI not found**
   - Run: `cargo install --locked soroban-cli`
   - Ensure `~/.cargo/bin` is in your PATH

4. **Contract size too large**
   - Use release profile: `--release`
   - Run soroban optimize command
   - Check for unnecessary dependencies

## Resources

- [Soroban Documentation](https://soroban.stellar.org/docs)
- [Stellar Documentation](https://developers.stellar.org/)
- [Soroban Examples](https://github.com/stellar/soroban-examples)
- [Rust Book](https://doc.rust-lang.org/book/)

## Contributing

See the main project [CONTRIBUTING.md](../CONTRIBUTING.md) for guidelines.

## License

MIT License - See [LICENSE](../LICENSE) for details.
