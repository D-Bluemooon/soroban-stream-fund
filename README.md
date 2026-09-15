# soroban-stream-fund

A minimal, auditable **per-second token streaming contract** for [Soroban](https://soroban.stellar.org), Stellar's smart contract platform.

## Why this exists

Continuous, streaming payments are one of the clearest funding primitives for open-source
maintainers and contributors: instead of a single lump-sum grant, a supporter can open a
stream that pays out gradually over time, and the recipient withdraws whatever has
accrued whenever they like. This pattern is already proven on Ethereum (e.g. Sablier,
Superfluid, Drips' own streaming model) but there isn't yet a small, easy-to-read
reference implementation of it on Soroban. This repo aims to be that reference: a
contract simple enough to read in one sitting, with tests, that other Stellar projects
(payroll tools, dependency-funding lists, contributor reward systems) can build on or
fork.

## What it does

- `create_stream(sender, recipient, token, deposit, rate_per_second) -> id`
  Deposits `deposit` tokens from `sender` into the contract and opens a stream that
  pays `rate_per_second` tokens to `recipient`.
- `withdraw(id) -> i128`
  Callable by the stream's recipient. Transfers everything accrued since the last
  withdrawal (capped at the remaining balance) and returns the amount sent.
- `balance(id) -> i128`
  Read-only. Returns the stream's remaining, not-yet-withdrawn balance.

## What it deliberately does *not* do (yet)

This is a starting point, not a finished product. See [`ISSUES.md`](./ISSUES.md) and the
repo's open issues for concrete, scoped next steps — including stream cancellation,
top-ups to an existing stream, and multi-recipient splitting.

## Getting started

```bash
# build
cargo build --target wasm32-unknown-unknown --release

# run tests
cargo test
```

Requires the `wasm32-unknown-unknown` target and the [Soroban CLI](https://soroban.stellar.org/docs)
if you want to deploy to a local network or testnet.

## Docs

- [`ARCHITECTURE.md`](./ARCHITECTURE.md) — how the contract is structured and why
- [`CONTRIBUTING.md`](./CONTRIBUTING.md) — how to propose changes and pick up an issue
- [`ISSUES.md`](./ISSUES.md) — the current open issues, mirrored here for convenience

## License

MIT — see [`LICENSE`](./LICENSE).
