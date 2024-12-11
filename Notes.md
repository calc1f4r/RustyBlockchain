# Project Notes

## Current State

### Runtime

- The `Runtime` struct initializes with `system` and `balances` pallets.
- The `Runtime` struct is defined in `src/main.rs`.

### System Pallet

- The `system::Pallet` struct manages `block_number` and `nonce`.
- Methods include:
  - `new()`: Initializes with `block_number` set to 0 and an empty `nonce` map.
  - `increment_block_number()`: Increments the block number.
  - `increment_nonce()`: Increments the nonce for a given user, with overflow handling.
  - `get_block_number()`: Returns the current block number.
  - `get_nonce()`: Returns the nonce for a given user.

### Balances Pallet

- The `balances::Pallet` struct manages user balances.
- Methods include:
  - `new()`: Initializes with an empty `balances` map.
  - `transfer()`: Transfers an amount from one user to another, with balance and overflow checks.
  - `set_balances()`: Sets the balance for a user.
  - `get_balance()`: Returns the balance for a user.

### Tests

- Tests for both `system` and `balances` pallets are included.
- Tests cover initialization, balance transfers, nonce increments, and overflow handling.

### Dependencies

- The project uses the `num` crate for numeric operations.
- Dependencies are listed in `Cargo.toml` and `Cargo.lock`.

### Miscellaneous

- The `.gitignore` file includes the `/target` directory.
