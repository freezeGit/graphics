---
sessionId: session-261009-123203-1um4
---

# Requirements

### Overview & Goals
The objective is to fix a compilation syntax error in `crates/app6/src/world/emerge.rs` where a stray `if` keyword causes `rustc` to fail with `error: expected '{', found ';'`.

### Scope
- **In Scope:**
  - Remove the extraneous `if` token on line 422 in `crates/app6/src/world/emerge.rs`.
  - Verify that `cargo check -p app6` and `cargo test -p app6` build and pass cleanly.
- **Out of Scope:**
  - Unrelated refactoring or modifying other crates within the workspace.

### Functional Requirements
- `crates/app6/src/world/emerge.rs` must contain valid Rust syntax without dangling keywords.
- `step_bg` must properly execute `interact_bits` followed by `change_cxn` whenever `i != j`.
- Crate `app6` must compile and pass all tests.

# Technical Design

### Current Implementation & Root Cause Analysis
In `crates/app6/src/world/emerge.rs` (lines 412–425), the `step_bg` function is defined as:

```rust
pub fn step_bg(bg: &mut BitGraph, bits_rule: BitsRule, cxns_rule: CxnsRule, rng: &mut impl Rng) {
    let n = bg.nodes();

    let i = rng.random_range(0..n);
    let j = rng.random_range(0..n);

    if i == j {
        return;
    }

    if
    interact_bits(&mut bg.values, bits_rule, i, j);
    change_cxn(bg, cxns_rule, i, j);
}
```

Line 422 has a dangling `if` keyword. In Rust, `if` starts an expression that requires a condition followed by a block `{ ... }`. The compiler interprets the function call `interact_bits(&mut bg.values, bits_rule, i, j)` as the condition expression and expects a `{` block immediately following it. Finding a `;` on line 423 instead triggers `error: expected '{', found ';'`.

### Proposed Changes
Remove the stray `if` keyword on line 422 so `step_bg` matches the pattern used across the codebase (e.g., `step_bg_rand` and `app5::world::emerge::step_bg`):

```rust
pub fn step_bg(bg: &mut BitGraph, bits_rule: BitsRule, cxns_rule: CxnsRule, rng: &mut impl Rng) {
    let n = bg.nodes();

    let i = rng.random_range(0..n);
    let j = rng.random_range(0..n);

    if i == j {
        return;
    }

    interact_bits(&mut bg.values, bits_rule, i, j);
    change_cxn(bg, cxns_rule, i, j);
}
```

### File Structure
- `crates/app6/src/world/emerge.rs`: Remove stray `if` at line 422.

# Testing

### Validation Approach
Verify the fix using standard Cargo compilation and testing commands.

### Key Scenarios
1. **Compilation Check**:
   - Run `cargo check -p app6` to ensure the compilation error is resolved.
2. **Unit Tests**:
   - Run `cargo test -p app6` to ensure all tests (including `bit_array_works`) compile and pass.

# Delivery Steps

### ✓ Step 1: Remove stray if keyword from step_bg in emerge.rs
The syntax error in `crates/app6/src/world/emerge.rs` is resolved by removing the dangling `if` token before `interact_bits`.

- Locate `step_bg` in `crates/app6/src/world/emerge.rs` around line 422.
- Remove the stray `if` keyword immediately preceding `interact_bits(&mut bg.values, bits_rule, i, j);`.
- Ensure `step_bg` sequentially executes `interact_bits` followed by `change_cxn` after the `if i == j` guard clause.

### ✓ Step 2: Validate compilation and test suite for app6
`app6` compiles cleanly with zero compilation errors and all crate unit tests pass.

- Run `cargo check -p app6` to verify syntax resolution and error-free compilation.
- Run `cargo test -p app6` to execute existing unit tests in `emerge.rs` and verify crate stability.