---
sessionId: session-260926-214803-fdar
---

# Requirements

### Overview & Goals
The `app5` crate currently fails to compile due to two naming mismatches in `crates/app5/src/canvas/seq_graph.rs`:
1. `error[E0425]: cannot find value 'ones_vec' in this scope` at line 77 in `SeqGraph::new`.
2. `error[E0609]: no field 'vec' on type '&SeqGraph'` at line 142 in `SeqGraph::draw_at`.

The goal is to fix these variable and field references so that `SeqGraph` matches its struct definition (`ones_vec: Vec<Rectangle>`) and `app5` compiles and tests successfully.

### Scope
- **In Scope**:
  - Rename the local vector in `SeqGraph::new` from `vec` to `ones_vec`.
  - Fix the field access in `SeqGraph::draw_at` from `&self.vec` to `&self.ones_vec`.
  - Validate crate compilation and tests for `app5`.
- **Out of Scope**:
  - Modifying `SeqGraph` logic or zoom calculations.
  - Refactoring other canvas components or unrelated warnings.

### Functional Requirements
- `SeqGraph::new` creates and populates `ones_vec` with `SG_SIZE` marker rectangles and successfully constructs a `SeqGraph` instance.
- `SeqGraph::draw_at` iterates through each rectangle marker in `self.ones_vec` and draws them to the painter.
- `cargo check -p app5` and `cargo test -p app5` succeed without compilation errors.


# Technical Design

### Current Implementation
In `crates/app5/src/canvas/seq_graph.rs`:
- Struct definition:
  ```rust
  pub struct SeqGraph {
      base: ShapeBase,
      ones_vec: Vec<Rectangle>,
      mid: Line,
      lines: Lines,
      zoom: Zoom,
  }
  ```
- In `SeqGraph::new` (lines 41-77):
  - Declares `let mut vec = Vec::new();` and pushes rectangles into `vec`.
  - Attempts to instantiate `Self` with `ones_vec` using shorthand field initialization, causing `E0425: cannot find value 'ones_vec' in this scope`.
- In `SeqGraph::draw_at` (lines 141-144):
  - Iterates over `&self.vec`, which does not exist on `SeqGraph`, causing `E0609: no field 'vec' on type '&SeqGraph'`.

### Key Decisions
- **Decision**: Rename the local vector in `SeqGraph::new` to `ones_vec` rather than writing `ones_vec: vec`.
  - *Rationale*: Maintains consistent naming throughout the function and fits idiomatic Rust field initialization shorthand `ones_vec`.
- **Decision**: Update `&self.vec` to `&self.ones_vec` in `SeqGraph::draw_at`.
  - *Rationale*: Directly references the existing `ones_vec: Vec<Rectangle>` field on `SeqGraph`.

### Proposed Changes
In `crates/app5/src/canvas/seq_graph.rs`:
1. In `SeqGraph::new`:
   - Replace `let mut vec = Vec::new();` with `let mut ones_vec = Vec::new();`.
   - In loop (line 53), change `vec.push(rect);` to `ones_vec.push(rect);`.
2. In `SeqGraph::draw_at`:
   - Replace `for s in &self.vec` with `for s in &self.ones_vec`.

### File Structure
- Modified: `crates/app5/src/canvas/seq_graph.rs`


# Testing

### Validation Approach
Verify that the compilation errors are resolved using `cargo check` and `cargo test`.

### Key Scenarios
- **Compilation Check**: Run `cargo check -p app5` to verify `crates/app5/src/canvas/seq_graph.rs` compiles cleanly without `E0425` or `E0609`.
- **Test Execution**: Run `cargo test -p app5` to verify existing tests in `app5` compile and pass.
- **Workspace Check**: Run `cargo check --workspace` to confirm workspace integrity.


# Delivery Steps

###   Step 1: Fix variable naming and struct initialization in SeqGraph::new
`SeqGraph::new` initializes the `ones_vec` field cleanly using a matching local variable.

- In `crates/app5/src/canvas/seq_graph.rs`, rename local variable `let mut vec = Vec::new();` to `let mut ones_vec = Vec::new();`.
- Update the loop body to push constructed `Rectangle` elements into `ones_vec.push(rect);`.
- Retain field shorthand `ones_vec` in struct initialization `Self { base, ones_vec, mid, lines, zoom: Zoom::default() }`.

###   Step 2: Fix field reference in SeqGraph::draw_at and verify build
`SeqGraph::draw_at` correctly iterates over `self.ones_vec` to render rectangles, and `app5` compiles without errors.

- In `crates/app5/src/canvas/seq_graph.rs`, update the loop in `SeqGraph::draw_at` from `for s in &self.vec` to `for s in &self.ones_vec`.
- Run `cargo check -p app5` and `cargo test -p app5` to verify that all compilation errors in `app5` are resolved.