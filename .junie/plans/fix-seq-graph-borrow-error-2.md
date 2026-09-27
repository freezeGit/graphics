---
sessionId: session-260927-103846-2yu9
---

# Requirements

### Overview & Goals
The project fails to compile in `crates/app5/src/canvas/seq_graph.rs` with error `E0502: cannot borrow *self as immutable because it is also borrowed as mutable`. The goal is to resolve this borrow checker error by reordering calculations so that immutable field reads on `self` (`self.location()` and `self.zoom`) happen before borrowing vector fields mutably, and to eliminate duplicated logic in `SeqGraph`.

### Scope
- **In Scope:**
  - Fix the borrow checker error in `crates/app5/src/canvas/seq_graph.rs` (`SeqGraph::add_val`).
  - Refactor `SeqGraph::add_ones_val` to delegate to `SeqGraph::add_val`.
  - Fix related warnings in `crates/app5/src/canvas/seq_graph.rs` (e.g. unused `mut` on `cxns_vec`).
- **Out of Scope:**
  - Changes to other crates or unrelated modules in `crates/app5`.
  - Modification of visual behavior or graph rendering logic.

### Functional Requirements
- `SeqGraph` methods (`add_val` and `add_ones_val`) must compile without errors.
- Adding values to `ones_vec` and `cxns_vec` must correctly shift existing elements to the left and update the newest sample point's vertical position based on current zoom settings and baseline location.

# Technical Design

### Current Implementation
In `crates/app5/src/canvas/seq_graph.rs`, `SeqGraph::add_val` is defined as:

```rust
fn add_val(&mut self, sg_vals: SeqGraphVals, ones_fraction: f32) {
    let mut vec_val = match sg_vals {
        SeqGraphVals::Ones => &mut self.ones_vec,
        SeqGraphVals::Cxns => &mut self.cxns_vec,
    };

    if vec_val.is_empty() {
        return;
    }

    for i in 0..vec_val.len() - 1 {
        let current_x = vec_val[i].location().x;
        let next_y = vec_val[i + 1].location().y;
        let new_location = egui::Pos2::new(current_x, next_y);
        vec_val[i].move_to(new_location);
    }

    let vx = vec_val.last_mut().unwrap().location().x;

    let clamped_fraction = ones_fraction.clamp(0.0, 1.0);
    let scaled_height =
        (0.5 + (clamped_fraction - self.zoom.focus) * self.zoom.scale) * SG_HEIGHT;
    let mark_offset = SG_MARK_SIZE / 2.0;
    let vy = self.location().y - (mark_offset + scaled_height);

    let loc = egui::Pos2::new(vx, vy);
    vec_val.last_mut().unwrap().move_to(loc);
}
```

### Root Cause Analysis
1. `vec_val` borrows `self.ones_vec` or `self.cxns_vec` mutably.
2. While `vec_val` is live (and later used on line 167 `vec_val.last_mut().unwrap().move_to(loc)`), line 164 calls `self.location()`.
3. `self.location()` calls a method taking `&self`, requiring an immutable borrow of the entire `*self`.
4. Rust forbids simultaneous active mutable and immutable borrows of the same struct instance `*self`.

### Key Decisions
- **Precompute position variables before mutable borrow:** Compute `clamped_fraction`, `scaled_height`, `mark_offset`, and `vy` using `self.location().y` and `self.zoom` at the top of `add_val`. After `vy` is computed as a primitive `f32`, acquire `vec_val` mutably to perform the shift and update the last element.
- **Deduplicate `add_ones_val`:** Have `add_ones_val` call `self.add_val(SeqGraphVals::Ones, ones_fraction)`.

### Proposed Changes
In `crates/app5/src/canvas/seq_graph.rs`:
1. Restructure `add_val`:
   ```rust
   fn add_val(&mut self, sg_vals: SeqGraphVals, ones_fraction: f32) {
       let clamped_fraction = ones_fraction.clamp(0.0, 1.0);
       let scaled_height =
           (0.5 + (clamped_fraction - self.zoom.focus) * self.zoom.scale) * SG_HEIGHT;
       let mark_offset = SG_MARK_SIZE / 2.0;
       let vy = self.location().y - (mark_offset + scaled_height);

       let vec_val = match sg_vals {
           SeqGraphVals::Ones => &mut self.ones_vec,
           SeqGraphVals::Cxns => &mut self.cxns_vec,
       };

       if vec_val.is_empty() {
           return;
       }

       for i in 0..vec_val.len() - 1 {
           let current_x = vec_val[i].location().x;
           let next_y = vec_val[i + 1].location().y;
           let new_location = egui::Pos2::new(current_x, next_y);
           vec_val[i].move_to(new_location);
       }

       let vx = vec_val.last_mut().unwrap().location().x;
       let loc = egui::Pos2::new(vx, vy);
       vec_val.last_mut().unwrap().move_to(loc);
   }
   ```
2. Simplify `add_ones_val`:
   ```rust
   pub fn add_ones_val(&mut self, ones_fraction: f32) {
       self.add_val(SeqGraphVals::Ones, ones_fraction);
   }
   ```
3. Remove unused `mut` from `let cxns_vec = Vec::new();` in `SeqGraph::new`.

### File Structure
- `crates/app5/src/canvas/seq_graph.rs` (modified)

# Testing

### Validation Approach
- Verify compilation of `crates/app5` via `mcp_rustrover_build_project` or `cargo check --bin app5`.
- Ensure no borrow checker errors or warnings remain in `seq_graph.rs`.

### Key Scenarios
- **Compilation verification:** Verify `cargo check` / `cargo build` succeeds for `app5`.
- **Runtime sequence update:** Verify that `add_ones_val` shifts sample points to the left and places the newly sampled point with the correct calculated `y` offset based on scale and focus.

# Delivery Steps

###   Step 1: Resolve borrow conflict in SeqGraph::add_val
The `add_val` method in `SeqGraph` compiles cleanly without borrow checker conflicts.

- Reorder operations in `crates/app5/src/canvas/seq_graph.rs`: compute `clamped_fraction`, `scaled_height`, `mark_offset`, and `vy` using `self.location().y`, `self.zoom.focus`, and `self.zoom.scale` before taking the mutable borrow `vec_val`.
- Remove unnecessary `mut` on `vec_val` (`let vec_val = match sg_vals ...`).
- Verify project compilation using the build tool.

###   Step 2: Refactor add_ones_val and clean up warnings
Code duplication between `add_ones_val` and `add_val` is eliminated and related compiler warnings in `seq_graph.rs` are cleaned up.

- Update `SeqGraph::add_ones_val` in `crates/app5/src/canvas/seq_graph.rs` to delegate directly to `self.add_val(SeqGraphVals::Ones, ones_fraction)`.
- Fix the unused `mut` warning on `let mut cxns_vec = Vec::new();` in `SeqGraph::new`.
- Run project build to ensure zero compiler errors and warnings in `crates/app5/src/canvas/seq_graph.rs`.