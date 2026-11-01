# Installation and local verification

This guide prepares a local checkout for Rust development and offline verification. It does not configure or authorize live gameplay automation.

## 1. Install prerequisites

Install Git and the current stable Rust toolchain from https://rustup.rs/.

On Debian/Ubuntu/WSL, install common native build prerequisites:

```bash
sudo apt update
sudo apt install -y build-essential pkg-config
```

On Windows, install the Rust MSVC toolchain and Visual Studio Build Tools with the C++ workload.

## 2. Clone the recovery branch

```bash
git clone --branch recovery/reconstruct-missing-modules https://github.com/narutosahu89-svg/myRaidlords-Bot.git
cd myRaidlords-Bot
```

## 3. Compile and run offline tests

```bash
cargo check --locked
cargo test --locked
cargo build --release --locked
```

The test suite does not contact the game service. A successful build is not evidence that the live API, wallet authentication, or game endpoints work.

## 4. Keep credentials out of Git

The repository intentionally contains no wallet private key, session token, signature, cookie, or production credential. Do not add any of those values to commits, issues, pull requests, or chat messages.

## Platform notes

- Linux/macOS: Bash scripts can be run from a terminal.
- Windows: use PowerShell for the Cargo commands; use WSL if you need Bash.
- `run.sh` is a convenience script, not a substitute for the Cargo build/test commands.

## Current integration status

The API/auth/RPC defaults are provisional and have not been verified against an official public API specification. The last reported live attempt returned `Not found` and HTTP `405`. Use only official API documentation and written authorization from the game operators for any permitted integration.
