# WCSPR

Wrapped CSPR (WCSPR) — a CEP-18 (ERC-20 style) wrapper for the native CSPR token on the
[Casper](https://casper.network) network, built with the [Odra](https://odra.dev) framework.

Wrapping native CSPR into a fungible token lets it be used by smart contracts and protocols
(DEXes, lending markets, etc.) that expect a standard token interface. Depositing CSPR mints an
equal amount of WCSPR, and withdrawing burns WCSPR to release the underlying CSPR.

## Contracts

This crate ships two contract versions. `WCSPRV2` is an upgrade of `WCSPRV1` that keeps the
original storage and adds gasless, signature-based authorization transfers.

### `WCSPRV1`

A CEP-18 compliant wrapped-token contract (see [src/wcspr_v1.rs](src/wcspr_v1.rs)).

- Token metadata: name `Wrapped CSPR`, symbol `WCSPR`, `9` decimals.
- `deposit` (payable) — wraps attached CSPR, minting an equal amount of WCSPR to the caller and
  emitting a `Deposit` event.
- `withdraw(amount)` — burns the caller's WCSPR and returns the native CSPR, emitting a
  `Withdrawal` event. If the caller is a contract, CSPR is delivered via `CsprDepositContractRef`.
- `withdraw_to(recipient, amount)` — burns the caller's WCSPR and sends the native CSPR directly
  to `recipient`, avoiding an intermediate transfer.
- Standard CEP-18 methods: `name`, `symbol`, `decimals`, `total_supply`, `balance_of`,
  `allowance`, `approve`, `increase_allowance`, `decrease_allowance`, `transfer`, `transfer_from`.

### `WCSPRV2`

An upgrade of `WCSPRV1` that composes the original token with the `CEP3009` module to add
authorization-based (gasless / meta) transfers (see [src/wcspr_v2.rs](src/wcspr_v2.rs)).

- `init(chain_name)` — initializes the underlying V1 token and the CEP-3009 module.
- `upgrade(chain_name)` — initializes the CEP-3009 module when upgrading from V1.
- Re-exposes all `WCSPRV1` methods (including `deposit`, `withdraw`, `withdraw_to`).
- Adds CEP-3009 methods:
  - `authorization_state(authorizer, nonce)`
  - `transfer_with_authorization(...)`
  - `receive_with_authorization(...)`
  - `cancel_authorization(...)`

## Project structure

```
src/
  lib.rs        # crate root, exposes wcspr_v1 and wcspr_v2 modules
  wcspr_v1.rs   # WCSPRV1 contract + unit tests
  wcspr_v2.rs   # WCSPRV2 contract (V1 + CEP-3009)
bin/
  build_contract.rs  # Wasm contract build entrypoint
  build_schema.rs    # contract schema generation entrypoint
  cli.rs             # Odra CLI: deploy script + upgrade scenario
Odra.toml     # registers WCSPRV1 and WCSPRV2 contracts
justfile      # common developer commands
```

## Prerequisites

- The Rust toolchain pinned in [rust-toolchain](rust-toolchain).
- The `wasm32-unknown-unknown` target: `rustup target add wasm32-unknown-unknown`.
- [`just`](https://github.com/casey/just) (optional, for the helper commands).

## Building

Build the Wasm contracts using the Odra build pipeline:

```bash
cargo odra build
```

Generate the contract schemas:

```bash
cargo odra schema
```

## Testing

Run the unit tests against the Odra MockVM:

```bash
cargo odra test
```

The tests in [src/wcspr_v1.rs](src/wcspr_v1.rs) cover initialization, deposits, minting,
withdrawals, and the relevant error cases.

## CLI

A CLI binary is provided to deploy the contract and run on-chain scenarios
(see [bin/cli.rs](bin/cli.rs)). Run it via the `justfile`:

```bash
just cli <ARGS>
```

or directly with cargo:

```bash
cargo run --bin wcspr_cli -- <ARGS>
```

### Deployment

`WCSPRV1DeployScript` deploys `WCSPRV1` as an upgradable contract registered under the package
named key `WrappedNativeToken`. This named key must match the one used on testnet and mainnet, so
it should not be changed.

### Upgrade scenario

The `upgrade_v1_to_v2` scenario upgrades a deployed `WCSPRV1` to `WCSPRV2`. It accepts a single
argument:

- `chain_name` — the name of the blockchain network, used to initialize the CEP-3009 module.
