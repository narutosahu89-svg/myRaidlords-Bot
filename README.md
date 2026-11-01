# RaidLords Rust project (recovery branch)

> **Status:** This repository is a reconstructed Rust codebase for build and offline-test work. It is **not a verified working live API client**. The configured service routes and wallet-authentication protocol have not been confirmed. The previous live test returned `Not found` and HTTP `405`.
>
> **Game rules:** The official RaidLords handbook states that bots, scripts, macros, and automated play are not allowed. Do not use this project to automate gameplay. Obtain written authorization and official API documentation from the game operators before attempting any integration.

## What is included

- Rust 2021 project, `Cargo.toml`, and committed `Cargo.lock`
- Reconstructed modules required by the current source tree
- Configuration examples and shell helper
- Offline Rust unit tests
- GitHub Actions build workflow

Some reconstructed modules are not upstream originals. Network defaults in `src/support/consts/net.rs` are provisional and must not be treated as official API documentation.

## Requirements

- Git
- Current stable Rust toolchain (rustup, including `cargo`)
- On Linux, the system build tools required by Rust dependencies

Install Rust from [rustup.rs](https://rustup.rs/).

## Clone this repository

The recovery work is on a branch, not on `main` yet:

```bash
git clone --branch recovery/reconstruct-missing-modules https://github.com/narutosahu89-svg/myRaidlords-Bot.git
cd myRaidlords-Bot
```

To inspect the default branch instead, omit `--branch recovery/reconstruct-missing-modules`.

## Build and verify locally

These commands compile and test the code; they do not verify live service access or game compatibility.

```bash
cargo fmt --check
cargo check --locked
cargo test --locked
cargo build --release --locked
```

On Windows PowerShell, run the same `cargo` commands. The Bash helper `run.sh` is intended for Linux, macOS, or WSL.

## Repository files

- `src/`: Rust source modules
- `assets/`: bundled project assets
- `Cargo.toml` / `Cargo.lock`: reproducible Rust dependency manifest and lockfile
- `.env.example`: placeholder only; contains no credentials
- `config.json`: example local configuration
- `proxy.txt`: optional empty proxy-list placeholder
- `run.sh` / `Makefile`: development helper scripts

Generated files such as `.env`, `target/`, session caches, device caches, and logs are ignored by Git. Never commit private keys, wallet signatures, session tokens, cookies, or other credentials.

## Known limitations

- Live API base, RPC, auth, version, and simulator routes are unverified.
- Wallet sign-in request/response schema is unverified.
- Passing Rust tests only confirms local code/test behavior; it does not prove live authentication, API compatibility, or game behavior.
- No private key or production credential is included in this repository.

For legitimate integration work, use only API documentation and written authorization supplied by the game operators.

## License

See [LICENSE](LICENSE).
