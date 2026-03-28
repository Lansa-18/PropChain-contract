# Commenting Standards

This document defines the commenting conventions for all PropChain contracts.
Every contributor must follow these rules to keep the codebase readable and
consistent.

## Rust Doc Comments (`///`)

All public functions, types, and trait implementations must have a `///` doc
comment immediately above the item. Use the following template for functions:

```rust
/// Registers a new property in the registry.
///
/// # Arguments
///
/// * `property_id` - The unique identifier for the property
///
/// # Returns
///
/// Returns `Result<(), Error>` indicating success or failure
```

### Rules

1. **First line** - A single sentence describing what the function does. Use
   present tense ("Returns", "Creates", "Checks") rather than imperative
   ("Return", "Create", "Check").
2. **Arguments section** - Required when the function takes parameters. List
   every parameter with a brief description.
3. **Returns section** - Required when the function returns a non-trivial type.
   Describe possible success and error outcomes.
4. **Extra context** - Add a paragraph between the summary and `# Arguments`
   when the function has non-obvious behavior (side effects, compliance checks,
   dividend recalculations, etc.).

### Minimal variant

Simple getters and boolean queries may use a single-line doc comment when the
function signature is self-explanatory:

```rust
/// Returns the management agent for a token, if one is assigned.
#[ink(message)]
pub fn get_management_agent(&self, token_id: TokenId) -> Option<AccountId> {
```

## Inline Comments

Use inline comments (`//`) sparingly and only when the code is not
self-documenting:

- **Algorithm steps** - Explain the "why", not the "what".
- **Scaling factors** - Document magic numbers (e.g. `1_000_000_000_000` for
  dividend precision).
- **Workarounds** - Note any ink! or Substrate quirks that influence the
  implementation.

Bad:

```rust
// Increment counter
self.counter += 1;
```

Good:

```rust
// 1e12 scaling factor preserves precision across small share balances
let scaling: u128 = 1_000_000_000_000;
```

## Enum Variants

Every variant in a public enum must carry a `///` doc comment:

```rust
pub enum Error {
    /// Property does not exist in the registry
    PropertyNotFound,
    /// Caller is not authorized for this operation
    Unauthorized,
}
```

## Struct Fields

Storage struct fields should have a `///` doc comment when the field name alone
is ambiguous:

```rust
#[ink(storage)]
pub struct PropertyBridge {
    /// Multi-signature bridge requests indexed by request ID
    bridge_requests: Mapping<u64, MultisigBridgeRequest>,
}
```

## Events

Each event struct should have a `///` doc comment explaining when the event is
emitted. Indexed (`#[ink(topic)]`) fields do not need individual comments if
their names match the storage fields they reference.

## What Not to Comment

- Do not add comments that restate the code (`// return the value` above
  `return value`).
- Do not leave `TODO`, `FIXME`, or `HACK` comments in merged code. Open an
  issue instead.
- Do not add commented-out code. Remove unused code entirely.
- Do not use block comments (`/* */`). Use `///` for doc comments and `//` for
  inline comments.

## Enforcement

- Run `cargo doc --no-deps` to verify that all public items have documentation.
  Warnings about missing docs indicate violations.
- Code reviewers should check that every new `#[ink(message)]` function includes
  a doc comment before approving a PR.
