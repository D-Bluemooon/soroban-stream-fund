# Open Issues

These mirror the issues filed on the repo's Issues tab, with the labels they should
carry. Labels follow two groups: a **type** label and a **Drips Wave complexity** label
(`complexity:trivial` = 100 pts, `complexity:medium` = 150 pts, `complexity:high` = 200
pts), matching the point tiers described in the Drips Wave maintainer docs.

---

### 1. Allow third-party submission of `withdraw` on the recipient's behalf

**Labels:** `enhancement`, `complexity:medium`

**Description**
Right now only the `recipient` address can call `withdraw`, because it requires
`recipient.require_auth()`. That's correct for authorization (only the recipient can
*authorize* the transfer) but it means the recipient must personally submit and pay the
fee for every withdrawal transaction.

Soroban's auth model allows a **pre-signed authorization** to be submitted by a
different account than the one it authorizes. Update `withdraw` (or add a
`withdraw_on_behalf_of`) so the recipient can sign an authorization off-chain and have
any relayer/keeper submit the transaction and cover the fee.

**Acceptance criteria**
- Recipient can still directly call `withdraw` as today.
- A relayer can submit a `withdraw` call carrying the recipient's authorization without
  the recipient submitting the transaction themselves.
- A test exercises both paths.

---

### 2. Add `cancel_stream` with a fair mid-stream settlement

**Labels:** `enhancement`, `complexity:high`

**Description**
There is currently no way for a sender to cancel a stream. Add a `cancel_stream(id)`
entry point, callable by the sender, that:
- pays the recipient everything accrued up to the cancellation moment (same accrual
  formula as `withdraw`), and
- refunds the remaining, un-accrued balance back to the sender.

This needs care around edge cases: canceling a stream that's already fully depleted,
canceling immediately after creation (recipient accrued nothing), and re-entrancy safety
around the two token transfers.

**Acceptance criteria**
- `cancel_stream` correctly splits funds between recipient (accrued) and sender
  (remainder) in all the edge cases above.
- Calling `withdraw` or `cancel_stream` again on an already-canceled stream fails
  cleanly rather than panicking with an unhelpful error.
- Tests cover: cancel immediately, cancel mid-stream, cancel after full depletion,
  double-cancel attempt.

---

### 3. Fix missing zero-rate validation message / add a small docs example

**Labels:** `bug`, `documentation`, `complexity:trivial`

**Description**
`create_stream` asserts `rate_per_second > 0` but the assertion message doesn't explain
*why* (a zero-rate stream would never pay out and would lock the deposit forever with no
withdrawal path). Improve the assertion message, and add a short "Quick example" section
to `README.md` showing a full `create_stream` → wait → `withdraw` sequence using the
Soroban CLI, so new contributors can try the contract against a local network without
reading the test file first.

**Acceptance criteria**
- Assertion message clearly states the reason for the restriction.
- `README.md` has a copy-pasteable CLI example.
