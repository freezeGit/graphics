---
sessionId: session-261019-113447-17nn
---

# Requirements

### Overview & Goals
The objective is to resolve the Rust compilation error `error[E0425]: cannot find value 'probability' in this scope` occurring in `crates/app6/src/world/emerge.rs` at line 144 during struct instantiation in `BitsRule::new`.

### Scope
- **In Scope:**
  - Fix the variable/field naming mismatch in `BitsRule::new` within `crates/app6/src/world/emerge.rs`.
  - Expose a getter method `probability(&self) -> f64` on `BitsRule` if needed for accessing the field.
  - Verify that workspace crates compile cleanly.
- **Out of Scope:**
  - Major refactoring of simulation logic or other `app` crates.
  - Changing GUI layouts or unrelated simulation rules.

### User Stories
- As a developer, I want `cargo check` and `cargo build` for `crates/app6` to succeed without missing identifier errors so that I can run and test the simulation.

### Functional Requirements
- `BitsRule::new(number: u8, random: bool, probability: f64) -> Self` must properly initialize the `probability` field of `BitsRule`.
- The `BitsRule` struct definition and constructor must remain consistent across call sites in `crates/app6/src/world.rs` and `crates/app6/src/app.rs`.

# Technical Design

### Current Implementation
In `crates/app6/src/world/emerge.rs` (lines 128–153):
```rust
#[derive(Debug, Clone, Copy)]
pub struct BitsRule {
    random: bool,
    probability: f64,
    number: u8,
    flags: [bool; 4],
}

impl BitsRule {
    pub fn new(number: u8, random: bool, percent: f64) -> Self {
        assert!(
            number < 16,
            "Rule number must be less than 16, got {number}"
        );

        Self {
            random,
            probability, // <-- ERROR: `probability` does not exist in scope; parameter is named `percent`
            number,
            flags: [
                Self::bit(number, 3),
                Self::bit(number, 2),
                Self::bit(number, 1),
                Self::bit(number, 0),
            ],
        }
    }
}
```

### Key Decisions
- **Parameter Naming:** Rename parameter `percent: f64` to `probability: f64` in `BitsRule::new`. This aligns the constructor parameter name directly with the field name `probability`, enabling Rust's field init shorthand syntax cleanly.
- **Getter Addition:** Add `pub fn probability(&self) -> f64 { self.probability }` to complement existing `random(&self)` and `number(&self)` accessors on `BitsRule`.

### Proposed Changes
- In `crates/app6/src/world/emerge.rs`:
  - Change `pub fn new(number: u8, random: bool, percent: f64) -> Self` to `pub fn new(number: u8, random: bool, probability: f64) -> Self`.
  - Retain `Self { random, probability, number, flags: [...] }`.
  - Add `pub fn probability(&self) -> f64` getter method.

### Data Models / Contracts
Updated `BitsRule` constructor and getter signature:
```rust
impl BitsRule {
    pub fn new(number: u8, random: bool, probability: f64) -> Self;
    pub fn random(&self) -> bool;
    pub fn probability(&self) -> f64;
    pub fn number(&self) -> u8;
}
```

### Risks
- **Risk:** Call sites relying on parameter name in documentation or naming conventions.
- **Mitigation:** In Rust, positional arguments are used at call sites (`BitsRule::new(b_rule, b_rand, 0.2)`), so renaming the parameter does not break external callers.

# Testing

### Validation Approach
Verify fix through compiler diagnostics and test suite execution.

### Key Scenarios
- `cargo check --package app6` succeeds with zero errors.
- `cargo test --package app6` passes all unit tests.

### Test Changes
- Ensure existing unit tests in `crates/app6/src/world/emerge.rs` continue to pass.

# Delivery Steps

###   Step 1: Fix parameter and field binding in BitsRule
The `BitsRule` struct in `crates/app6/src/world/emerge.rs` compiles successfully without scope resolution errors.

- Update `BitsRule::new` signature or field initialization in `crates/app6/src/world/emerge.rs` so that the constructor parameter `probability: f64` matches the struct field `probability` (or assign `probability: percent`).
- Add a getter method `pub fn probability(&self) -> f64` on `BitsRule` to allow reading the configured probability value.

###   Step 2: Verify compilation and unit tests
The project builds cleanly and all unit tests in the workspace pass.

- Run `cargo check` and `cargo test` across `crates/app6` to verify no compilation errors remain.
- Verify that `BitsRule::new` instantiations in `crates/app6/src/world.rs` and `crates/app6/src/app.rs` compile without type or signature mismatch.