# Blueshift Vault

A lightweight Solana program for managing personal SOL vaults, built with [Pinocchio](https://github.com/anza-xyz/pinocchio) for minimal dependencies and optimized performance.

## What it does

This program lets you create your own personal vault on Solana where you can deposit and withdraw SOL. Think of it as a simple savings account, but on-chain and completely trustless.

Each user gets their own vault derived from their wallet address, so there's no risk of conflicts or accessing someone else's funds. The vault is a Program Derived Address (PDA) that only your wallet can control.

## Features

- **Deposit SOL**: Transfer SOL from your wallet to your personal vault
- **Withdraw SOL**: Take out all your SOL from the vault back to your wallet  
- **Zero dependencies**: Built with Pinocchio for minimal bloat
- **Gas efficient**: Optimized for low transaction costs

## Program Instructions

### Deposit (Discriminator: 0)
```rust
// Deposits a specified amount of SOL into your vault
// Data: [amount: u64] (8 bytes, little-endian)
```

### Withdraw (Discriminator: 1)  
```rust
// Withdraws all SOL from your vault
// Data: [] (no additional data needed)
```

## Building

You'll need the Solana CLI tools installed. If you don't have them:

```bash
sh -c "$(curl -sSfL https://release.solana.com/stable/install)"
```

Then build the program:

```bash
cargo build-sbf
```

The compiled program will be in `target/deploy/`.

## Account Structure

**Deposit Accounts:**
1. `owner` - Your wallet (signer)
2. `vault` - Your PDA vault (derived from your pubkey)
3. `system_program` - Solana system program

**Withdraw Accounts:**
1. `owner` - Your wallet (signer) 
2. `vault` - Your PDA vault
3. `system_program` - Solana system program

## Vault Derivation

Your vault address is derived using:
```
find_program_address(&[b"vault", owner.key().as_ref()], &program_id)
```

This ensures each wallet has exactly one vault and the address is deterministic.

## Security Notes

- Only the vault owner can deposit or withdraw
- Withdraws transfer the entire vault balance (no partial withdrawals)
- The program validates all account ownership and signatures
- Vault must be empty on first deposit (prevents overwriting existing vaults)

## License

This project is open source. Feel free to fork, modify, or use it as a reference for your own Solana programs.

---

*Built with ❤️ and Pinocchio* 