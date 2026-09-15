# Architecture

## Overview

The contract has a single storage-backed entity, `Stream`, and three entry points.
There is no external dependency beyond the standard Soroban `token` client interface
(`soroban_sdk::token`), which lets this contract move any Stellar Asset Contract (SAC)
or standard Soroban token without needing to know its internals.

```
                 create_stream
   sender  ───────────────────────▶  ┌───────────────────┐
                deposit tokens       │  StreamFundContract│
                                     │                    │
   recipient ◀─────────────────────  │  storage:          │
                withdraw()          │   Stream { .. }     │
                (accrued amount)    └───────────────────┘
```

## Data model

```rust
pub struct Stream {
    pub sender: Address,
    pub recipient: Address,
    pub token: Address,
    pub rate_per_second: i128,
    pub last_withdrawn: u64,   // ledger timestamp, seconds
    pub balance: i128,         // remaining undistributed deposit
}
```

Each stream is stored under `DataKey::Stream(id)` in **persistent** storage (it needs to
survive indefinitely, unlike `instance` or `temporary` storage). A single
`DataKey::NextId` counter in `instance` storage hands out monotonically increasing ids.

## Accrual model

Accrual is computed lazily, on withdrawal, rather than via a background process (Soroban
has no cron/keeper primitive, so this is the standard pattern):

```
elapsed = now - stream.last_withdrawn
owed    = elapsed * rate_per_second
payout  = min(owed, stream.balance)
```

This means a stream's payout is always exact, deterministic, and requires no off-chain
keeper to "tick" the contract — the recipient (or anyone acting on their behalf, once
issue #2 below lands) simply calls `withdraw` whenever they want funds.

## Authorization

- `create_stream` requires `sender.require_auth()` — only the sender can initiate a
  stream and move their own funds into escrow.
- `withdraw` requires `recipient.require_auth()` — only the intended recipient can pull
  funds out. See [issue: allow third-party withdrawal calls](#) in `ISSUES.md` for a
  planned relaxation of this (recipient-signed, anyone-submitted).

## Token transfers

All token movement goes through `soroban_sdk::token::Client`, which targets the
[SEP-41](https://github.com/stellar/stellar-protocol/blob/master/ecosystem/sep-0041.md)
token interface. This means the contract works unmodified with the native XLM SAC, any
issued asset's SAC, or any custom Soroban token that implements SEP-41 — including USDC
on Stellar, which is the settlement asset Drips Wave itself uses for reward payouts.

## Known limitations (tracked as issues)

1. No cancellation / refund path for the sender.
2. No ability to top up an existing stream without creating a new one.
3. No batch/multi-recipient splitting (a single deposit funding several streams at once).

These are intentionally left as open, scoped issues — see `ISSUES.md` — rather than
built up front, so the base contract stays small enough to audit quickly.
