# Health Checks

This document describes the health check system for PropChain contracts.

## Overview

PropChain includes health check capabilities at two levels:

1. **On-chain health check endpoints** built into the `PropertyRegistry` contract
2. **Off-chain monitoring** via scripts and CI workflows

## On-Chain Health Check Endpoints

The `PropertyRegistry` contract exposes three ink! messages for health monitoring.

### `ping()`

Simple liveness check. Returns `true` if the contract is deployed and responsive.

```rust
#[ink(message)]
pub fn ping(&self) -> bool
```

Use this for basic uptime monitoring and load balancer health probes.

### `health_check()`

Returns a `HealthStatus` struct with full contract state information.

```rust
#[ink(message)]
pub fn health_check(&self) -> HealthStatus
```

The `HealthStatus` struct includes:

| Field                      | Type   | Description                                  |
|----------------------------|--------|----------------------------------------------|
| `is_healthy`               | `bool` | `true` if the contract is not paused         |
| `is_paused`                | `bool` | Whether the contract is currently paused     |
| `contract_version`         | `u32`  | Current contract version number              |
| `property_count`           | `u64`  | Total registered properties                  |
| `escrow_count`             | `u64`  | Total escrow entries                         |
| `has_oracle`               | `bool` | Whether the oracle dependency is configured  |
| `has_compliance_registry`  | `bool` | Whether compliance registry is configured    |
| `has_fee_manager`          | `bool` | Whether the fee manager is configured        |
| `block_number`             | `u32`  | Current block number at query time           |
| `timestamp`                | `u64`  | Current block timestamp at query time        |

### `dependencies_healthy()`

Returns `true` only if all three critical dependencies are configured:

- Oracle contract
- Compliance registry contract
- Fee manager contract

```rust
#[ink(message)]
pub fn dependencies_healthy(&self) -> bool
```

## Off-Chain Health Check Script

The `scripts/health-check.sh` script performs comprehensive off-chain checks.

### Usage

```bash
# Run all checks
./scripts/health-check.sh

# Skip test execution (faster)
./scripts/health-check.sh --skip-tests

# Skip build verification
./scripts/health-check.sh --skip-build
```

### What It Checks

- **Toolchain**: Rust, Cargo, cargo-contract, WASM target
- **Dependencies**: ink!, scale-codec, and scale-info versions
- **Formatting**: `cargo fmt --check` compliance
- **Build**: Workspace and individual contract compilation
- **Tests**: Workspace test suite execution

The script generates a timestamped report file in the workspace root.

## Automated Health Monitoring

A GitHub Actions workflow runs daily at 06:00 UTC and can also be triggered manually.

### Workflow: `.github/workflows/health-check.yml`

The workflow performs:

1. Format check (`cargo fmt --all -- --check`)
2. Workspace compilation check (`cargo check --workspace`)
3. Clippy linting (`cargo clippy --all-targets --all-features`)
4. Test execution
5. Dependency version reporting
6. Per-contract health verification

### Manual Trigger

Navigate to Actions > Health Check > Run workflow in the GitHub UI, or use the CLI:

```bash
gh workflow run health-check.yml
```

## Monitoring Recommendations

- Use `ping()` for high-frequency liveness probes (every 30 seconds)
- Use `health_check()` for periodic detailed status (every 5 minutes)
- Use `dependencies_healthy()` after deployment to verify configuration
- Run the CI workflow daily to catch build regressions early
- Use the local script before releases to verify system health
