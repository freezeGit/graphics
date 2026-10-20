---
sessionId: session-261019-182324-scob
---

# Requirements

### Overview & Goals
To maximize system reliability, reduce developer friction, and eliminate failure modes, all latent defects, compilation errors, runtime panic conditions, and misleading documentation in `crates/app6/src/world.rs` and its associated interfaces must be identified and resolved. 

### Scope
- **In Scope**:
  - `crates/app6/src/world.rs`: Analysis of imports, struct definitions, constructor logic, trait implementation, and documentation.
  - Interfacing contracts in `crates/app6/src/inits.rs` and `crates/app6/src/world/emerge.rs` affecting `TheWorld`.
- **Out of Scope**:
  - Unrelated UI widgets or canvas rendering logic in `crates/app6/src/canvas.rs` and `crates/app6/src/app.rs`.
  - Refactoring unrelated crates (`app2`–`app5`, `demo`).

### Identified Problems & Functional Requirements
1. **Compilation Failure (E0432 & E0425)**:
   - `world.rs` imports `INITIAL_cxn_RULE_PROBABILITY` with lowercase `cxn`, while `inits.rs` defines `pub const INITIAL_CXN_RULE_PROBABILITY`.
   - In `TheWorld::new()`, `INITIAL_CXN_RULE_PROBABILITY` is referenced when calling `CxnsRule::new`, causing an unresolved name error.
   - *Requirement*: Unify identifier casing to `INITIAL_CXN_RULE_PROBABILITY` across imports and usages.

2. **Runtime Panic on Instantiation (Fatal Crash)**:
   - `inits.rs` sets `pub const INITIAL_CXN_RULE: u8 = 4;`.
   - `CxnsRule::new` asserts `assert!(number < 4, "Connections Rule number must be less than 4, got {number}");`.
   - Calling `TheWorld::new()` immediately panics, rendering the application inoperable upon startup.
   - *Requirement*: Harmonize the rule boundary assertion in `CxnsRule::new` (or set `INITIAL_CXN_RULE` within 0..=3) to prevent panics.

3. **State Inconsistency in BitGraph Initialization**:
   - `TheWorld::new()` initializes `start_ones: INITIAL_ONES` and `start_cxns: INITIAL_CXNS`, but instantiates `bit_graph: BitGraph::new(INITIAL_BITS_NUM)` (which zeroes all nodes and connections regardless of `INITIAL_ONES` and `INITIAL_CXNS`).
   - *Requirement*: Use `BitGraph::new_with_random_values(INITIAL_BITS_NUM, INITIAL_ONES, INITIAL_CXNS, &mut rng)` when initializing `TheWorld` to ensure state coherence with `start_ones` and `start_cxns`.

4. **Dead Code and Clutter**:
   - `BitArray` is imported into `world.rs` from `emerge.rs` but never referenced directly.
   - Stale commented-out code blocks add noise and hinder maintainability.
   - *Requirement*: Clean up unused imports and obsolete comments.

5. **Misleading Documentation / Architectural Contradictions**:
   - Doc comments claim `TheWorld` deliberately has no dependency on `gui_lib`, yet `use gui_lib::World;` and `impl World for TheWorld` are present.
   - File header comments still refer to `src/demo/world.rs` and `app1 struct`.
   - *Requirement*: Update documentation to accurately reflect actual crate boundaries and trait implementations.

# Technical Design

### Current Implementation
In `crates/app6/src/world.rs`:
```rust
use crate::inits::{
    INITIAL_BITS_NUM, INITIAL_BITS_RULE, INITIAL_BITS_RULE_RANDOM, INITIAL_BITS_RULE_PROBABILITY,
    INITIAL_CXN_RULE, INITIAL_CXNS, INITIAL_ONES, INITIAL_CXN_RULE_RANDOM, INITIAL_cxn_RULE_PROBABILITY
};
use crate::world::emerge::{BitArray, BitGraph, BitsRule, CxnsRule, step_bg};
use gui_lib::World;
use rand::rngs::SmallRng;
use rand::SeedableRng;

pub struct TheWorld {
    pub rng: SmallRng,
    pub bit_graph: BitGraph,
    pub bits_rule: BitsRule,
    pub cxns_rule: CxnsRule,
    pub start_ones: usize,
    pub start_cxns: usize,
    pub frame_number: u64,
}
```

### Detailed Problem Breakdown

#### 1. Import Case Mismatch (Compile-Time Failure)
- **Line 17**: `INITIAL_cxn_RULE_PROBABILITY` is imported with lowercase `cxn`.
- **Line 67**: `CxnsRule::new(INITIAL_CXN_RULE, INITIAL_CXN_RULE_RANDOM, INITIAL_CXN_RULE_PROBABILITY)` references the uppercase identifier.
- **Impact**: Compiler fails immediately (`error[E0432]: unresolved import`, `error[E0425]: cannot find value`).

#### 2. Assertion Mismatch / Runtime Panic
- In `crates/app6/src/inits.rs`:
  ```rust
  pub const INITIAL_CXN_RULE: u8 = 4; // must be 0 to 3 inclusive
  ```
- In `crates/app6/src/world/emerge.rs`:
  ```rust
  pub fn new(number: u8, random: bool, probability: f64) -> Self {
      assert!(
          number < 4,
          "Connections Rule number must be less than 4, got {number}"
      );
      Self { random, probability, number }
  }
  ```
- **Impact**: `TheWorld::new()` passes `INITIAL_CXN_RULE` (= 4), failing the assertion and crashing the process. Notice `change_cxn` supports rule `4` (random connection), but `CxnsRule::new` panics if `number >= 4`. Either `number < 5` should be permitted in `CxnsRule::new` or `INITIAL_CXN_RULE` should be `< 4`.

#### 3. Incomplete BitGraph Initialization
- `TheWorld::new()` creates `bit_graph` via `BitGraph::new(INITIAL_BITS_NUM)`.
- If a user configures `INITIAL_ONES > 0` or `INITIAL_CXNS > 0` in `inits.rs`, those values are recorded in `self.start_ones` and `self.start_cxns`, but the underlying `bit_graph` starts completely empty (all 0s).
- In `app.rs`, resets correctly call `BitGraph::new_with_random_values(bits, strt_ones, strt_cxns, &mut self.world.rng)`.

#### 4. Unused Imports & Obsolete Comments
- `BitArray` is never used in `world.rs` (`BitGraph` already wraps it).
- Doc comments say `It deliberately has no dependency on gui_lib or egui`, directly contradicting `impl World for TheWorld`.

### Proposed Corrected Code for `world.rs`
```rust
//! Program state and simulation logic.
//!
//! This module defines `TheWorld` and implements the `gui_lib::World` trait.

pub mod emerge;

use crate::inits::{
    INITIAL_BITS_NUM, INITIAL_BITS_RULE, INITIAL_BITS_RULE_PROBABILITY, INITIAL_BITS_RULE_RANDOM,
    INITIAL_CXNS, INITIAL_CXN_RULE, INITIAL_CXN_RULE_PROBABILITY, INITIAL_CXN_RULE_RANDOM,
    INITIAL_ONES,
};
use crate::world::emerge::{BitGraph, BitsRule, CxnsRule, step_bg};
use gui_lib::World;
use rand::rngs::SmallRng;
use rand::SeedableRng;

/// TheWorld struct encapsulates application data and simulation logic.
pub struct TheWorld {
    pub rng: SmallRng,
    pub bit_graph: BitGraph,
    pub bits_rule: BitsRule,
    pub cxns_rule: CxnsRule,
    pub start_ones: usize,
    pub start_cxns: usize,
    pub frame_number: u64,
}

impl World for TheWorld {
    /// Advance simulation by one step.
    fn advance(&mut self) {
        self.frame_number += 1;
        step_bg(
            &mut self.bit_graph,
            self.bits_rule,
            self.cxns_rule,
            &mut self.rng,
        );
    }
}

impl TheWorld {
    pub fn new() -> Self {
        let mut rng = SmallRng::from_rng(&mut rand::rng());
        let bit_graph = BitGraph::new_with_random_values(
            INITIAL_BITS_NUM,
            INITIAL_ONES,
            INITIAL_CXNS,
            &mut rng,
        );

        Self {
            rng,
            bit_graph,
            bits_rule: BitsRule::new(
                INITIAL_BITS_RULE,
                INITIAL_BITS_RULE_RANDOM,
                INITIAL_BITS_RULE_PROBABILITY,
            ),
            cxns_rule: CxnsRule::new(
                INITIAL_CXN_RULE,
                INITIAL_CXN_RULE_RANDOM,
                INITIAL_CXN_RULE_PROBABILITY,
            ),
            start_ones: INITIAL_ONES,
            start_cxns: INITIAL_CXNS,
            frame_number: 0,
        }
    }
}
```

# Testing

### Validation Approach
Verification focuses on confirming complete build success, runtime stability, and state consistency.

### Key Scenarios
1. **Compilation Check**:
   - Run `cargo check -p app6` to verify that all unresolved import errors (`E0432`) and missing variable errors (`E0425`) are eliminated.
   - Run `cargo check --workspace` to ensure zero compilation errors across the workspace.

2. **Constructor Panic Validation**:
   - Instantiate `TheWorld::new()` in unit/integration tests to ensure no assertion failures or panics occur with default `INITIAL_CXN_RULE` and other constants from `inits.rs`.

3. **Graph Initialization Verification**:
   - Verify that when `INITIAL_ONES` and `INITIAL_CXNS` are configured, `TheWorld::new()` produces a `bit_graph` containing the expected initial count of ones and connections.

4. **Simulation Step Verification**:
   - Call `world.advance()` for multiple iterations to ensure `self.frame_number` increments monotonically and `step_bg` executes without out-of-bounds indexing or runtime exceptions.

# Delivery Steps

###   Step 1: Resolve Compilation Errors and Import Inconsistencies
Eliminate all blocking compilation errors and unused imports in `crates/app6/src/world.rs`.

- Correct the casing of `INITIAL_cxn_RULE_PROBABILITY` to `INITIAL_CXN_RULE_PROBABILITY` in the `use crate::inits::{...}` import declaration in `crates/app6/src/world.rs`.
- Ensure `INITIAL_CXN_RULE_PROBABILITY` matches the identifier passed into `CxnsRule::new` on line 67.
- Remove unused imports (`BitArray`, unused commented-out dependencies) to prevent compiler warnings and minimize cognitive overhead.

###   Step 2: Fix Runtime Panic and Rule Range Invariants
Align rule boundary definitions between `inits.rs`, `emerge.rs`, and `world.rs` to prevent runtime panics upon instantiation.

- Adjust `INITIAL_CXN_RULE` in `crates/app6/src/inits.rs` or update the upper-bound validation assertion in `CxnsRule::new` in `crates/app6/src/world/emerge.rs` so that rule values (0..=4) are accepted consistently without triggering an immediate panic.
- Align the comments and boundary documentation in `inits.rs` and `emerge.rs` to reflect the valid range of connection rules.

###   Step 3: Synchronize Initial State and Rectify Module Contracts
Ensure `TheWorld::new()` properly initializes its internal `BitGraph` with configured start parameters and accurate documentation.

- Update `TheWorld::new()` to initialize `bit_graph` using `BitGraph::new_with_random_values(INITIAL_BITS_NUM, INITIAL_ONES, INITIAL_CXNS, &mut rng)` instead of `BitGraph::new(INITIAL_BITS_NUM)` when non-zero initial ones or connections are requested, ensuring internal state consistency.
- Correct the misleading doc comments in `crates/app6/src/world.rs` that state the module has no dependency on `gui_lib`, accurately reflecting its implementation of `gui_lib::World`.
- Remove legacy copy-paste comments referencing `src/demo/world.rs` and `app1`.
- Verify the entire workspace builds cleanly with `cargo check --workspace` and test execution.