# Contributing

Thanks for considering a contribution. This project is part of the Stellar open-source
ecosystem and participates in [Drips Wave](https://www.drips.network/wave/stellar).

## Ground rules

- Keep the base contract small. New capabilities should be additive and well-tested,
  not rewrites of existing behavior, unless an issue explicitly scopes a refactor.
- Every change to `src/lib.rs` needs a corresponding test in `src/test.rs`.
- Run `cargo fmt` and `cargo clippy --all-targets` before opening a PR.
- Follow [Conventional Commits](https://www.conventionalcommits.org/) for commit
  messages (`feat:`, `fix:`, `docs:`, `test:`, `chore:`).

## Picking up an issue

1. Check the [open issues](./ISSUES.md) or the repo's Issues tab.
2. Comment on the issue to claim it before starting work, so effort isn't duplicated.
3. Issues are labeled by complexity, matching the Drips Wave points system:
   - `complexity:trivial` — small, well-scoped fixes (typos, minor bugs, small docs gaps)
   - `complexity:medium` — a standard feature or non-trivial bug fix
   - `complexity:high` — a new capability, refactor, or contract-level change
4. Open a draft PR early if you want feedback on direction before finishing.

## PR checklist

- [ ] `cargo build --target wasm32-unknown-unknown --release` succeeds
- [ ] `cargo test` passes
- [ ] `cargo clippy --all-targets -- -D warnings` is clean
- [ ] New behavior has a test
- [ ] `README.md` / `ARCHITECTURE.md` updated if public behavior changed

## Reporting bugs or proposing features

Please open an issue first for anything beyond a trivial fix, describing:
- what's happening today vs. what you'd expect,
- why it matters (for a bug) or what it unlocks (for a feature),
- a rough sketch of the approach, if you have one.

This keeps scope-creep out of PRs and makes review faster for everyone.

## Code of conduct

Be respectful, assume good faith, and keep discussion focused on the work. Disagreements
about approach are fine and expected — personal attacks are not.
