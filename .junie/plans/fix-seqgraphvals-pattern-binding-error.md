---
sessionId: session-260927-100643-1gof
---

# Requirements

### Overview & Goals
When compiling `crates/app5`, rustc fails with `error[E0170]: pattern binding 'Ones' is named the same as one of the variants of the type 'canvas::seq_graph::SeqGraphVals'`.
The goal of this task is to explain the problem clearly and implement the proper fix in `crates/app5/src/canvas/seq_graph.rs` so that `app5` compiles without pattern binding errors.

### Why this happens
In Rust, enum variants are not automatically in scope unless brought into scope via `use SeqGraphVals::*;` or qualified by their type name `SeqGraphVals::Ones`. When `match sg_vals` is written as:
```rust
let val = match sg_vals {
    Ones => ones_fraction,
    Cxns => ones_fraction,
};
```
the compiler interprets `Ones` and `Cxns` not as variant patterns, but as catch-all variable bindings named `Ones` and `Cxns`. Because `Ones` happens to match the name of a variant of `SeqGraphVals`, rustc triggers `#[deny(bindings_with_variant_name)]` (error `E0170`) to prevent subtle bugs.

### Scope
- **In Scope**:
  - Fix pattern matching in `crates/app5/src/canvas/seq_graph.rs` (`SeqGraph::add_val`).
  - Verify compilation using `cargo check -p app5` and `cargo test -p app5`.
- **Out of Scope**:
  - Refactoring unrelated modules in `gui_lib` or other app crates.
  - Addressing general unused variable/import warnings outside the target function.

### Functional Requirements
- `SeqGraphVals` pattern matching in `SeqGraph::add_val` must correctly match `SeqGraphVals::Ones` and `SeqGraphVals::Cxns`.
- `crates/app5` must compile successfully with zero `E0170` errors.

# Technical Design

### Current Implementation
In `crates/app5/src/canvas/seq_graph.rs`:
```rust
enum SeqGraphVals {
    Ones,
    Cxns,
}
```
In `SeqGraph::add_val` (lines 141-150):
```rust
fn add_val(&mut self, sg_vals: SeqGraphVals, ones_fraction: f32) {
    // let val = match sg_vals {
    //     SeqGraphVals::Ones => ones_fraction,
    //     SeqGraphVals::Cxns => ones_fraction,
    // };
    let val = match sg_vals {
        Ones => ones_fraction,
        Cxns => ones_fraction,
    };
    ...
}
```
The active `match` statement uses bare identifiers, causing `E0170`.

### Key Decisions
- Qualify the enum variants explicitly with `SeqGraphVals::Ones` and `SeqGraphVals::Cxns` in the `match` block.
- Remove the commented-out block to keep the code clean and maintainable.

### Proposed Changes
Update `crates/app5/src/canvas/seq_graph.rs`:
```rust
fn add_val(&mut self, sg_vals: SeqGraphVals, ones_fraction: f32) {
    let val = match sg_vals {
        SeqGraphVals::Ones => ones_fraction,
        SeqGraphVals::Cxns => ones_fraction,
    };
    ...
}
```

### File Structure
- `crates/app5/src/canvas/seq_graph.rs` (modified)

# Testing

### Validation Approach
- Execute `cargo check -p app5` to verify that `error[E0170]` no longer occurs.
- Execute `cargo test -p app5` to verify compilation and test execution.

### Key Scenarios
- Compilation of `app5` crate succeeds with `cargo check -p app5`.
- Full workspace check succeeds with `cargo check --workspace`.

# Delivery Steps

###   Step 1: Fix enum variant matching in SeqGraph::add_val
The `SeqGraph::add_val` match block correctly qualifies `SeqGraphVals` variants and resolves compiler error E0170.

- Open `crates/app5/src/canvas/seq_graph.rs`.
- Update the `match sg_vals` block in `SeqGraph::add_val` to use `SeqGraphVals::Ones` and `SeqGraphVals::Cxns` instead of bare identifiers `Ones` and `Cxns`.
- Clean up the outdated commented match block in `add_val`.

###   Step 2: Validate crate compilation and tests
The `app5` crate compiles cleanly without pattern binding errors.

- Run `cargo check -p app5` to verify that `error[E0170]` is eliminated.
- Run `cargo test -p app5` to ensure all tests build and pass across the workspace.