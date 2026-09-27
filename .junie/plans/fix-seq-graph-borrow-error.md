---
sessionId: session-260927-102408-1xbe
---

# Requirements

### Overview & Goals
The objective is to explain and resolve the Rust borrow checker compilation error `E0596` (`cannot borrow '*vec_val' as mutable, as it is behind a '&' reference`) occurring in `crates/app5/src/canvas/seq_graph.rs:155:13`, as well as fix the underlying logic errors in `SeqGraph::add_val` and `SeqGraph::new`.

### Scope
- **In Scope**:
  - Explain why Rust emitted error `E0596` on `vec_val[i].move_to(new_location)`.
  - Fix the match expression in `SeqGraph::add_val` so both branches return a mutable reference `&mut Vec<Rectangle>`.
  - Correct the target vector in `SeqGraphVals::Cxns` to `&mut self.cxns_vec` instead of `&self.ones_vec`.
  - Update the final rectangle positioning on line 167 to mutate `vec_val` rather than `self.ones_vec`.
  - Refactor `add_ones_val` to delegate to `add_val` and expose `add_cxns_val`.
  - Ensure `cxns_vec` is initialized and drawn consistently.
- **Out of Scope**:
  - Modifying simulation logic in other crates (`app1`–`app4`, `gui_lib`).

### Functional Requirements
- `crates/app5` must compile cleanly with `cargo check --all-targets --workspace`.
- Calling `add_ones_val` or `add_cxns_val` must correctly shift existing points and position the latest value in `ones_vec` or `cxns_vec` respectively.

# Technical Design

### Current Implementation & Root Cause
In `crates/app5/src/canvas/seq_graph.rs`:
```rust
fn add_val(&mut self, sg_vals: SeqGraphVals, ones_fraction: f32) {
    let mut vec_val = match sg_vals {
        SeqGraphVals::Ones => &mut self.ones_vec,
        SeqGraphVals::Cxns => &self.ones_vec,
    };
    ...
    vec_val[i].move_to(new_location);
    ...
    self.ones_vec.last_mut().unwrap().move_to(loc);
}
```

#### Why Error E0596 Occurs:
1. **Type Coercion to Immutable Reference**: In the `match` expression, the first arm produces `&mut Vec<Rectangle>` while the second arm produces `&Vec<Rectangle>`. Because both arms must have a compatible type, Rust coerces the overall expression type to the shared (immutable) reference `&Vec<Rectangle>`.
2. **Cannot Borrow as Mutable**: Although `vec_val` is declared with `let mut`, `vec_val` holds an immutable reference `&Vec<Rectangle>`. Calling `vec_val[i].move_to(...)` and `vec_val.last_mut()` requires mutable dereferencing (`&mut Rectangle`), which is prohibited through a shared reference.
3. **Logic Bugs**:
   - `SeqGraphVals::Cxns` incorrectly pointed to `self.ones_vec` instead of `self.cxns_vec`.
   - Line 167 mutated `self.ones_vec` directly rather than `vec_val`, meaning updating `Cxns` would still mutate the wrong vector.
   - In `SeqGraph::new`, `cxns_vec` was created empty without the initial `SG_SIZE` markers.

### Key Decisions
- **Unified Mutable Access**: Match arms must return `&mut self.ones_vec` and `&mut self.cxns_vec` respectively, yielding a `&mut Vec<Rectangle>` for `vec_val`.
- **Target `vec_val` for Tail Update**: Replace `self.ones_vec.last_mut().unwrap().move_to(loc)` with `vec_val.last_mut().unwrap().move_to(loc)`.
- **DRY Delegation**: Have `pub fn add_ones_val(&mut self, ones_fraction: f32)` delegate directly to `self.add_val(SeqGraphVals::Ones, ones_fraction)`.

### Proposed Changes
In `crates/app5/src/canvas/seq_graph.rs`:
1. Update `SeqGraph::new`:
   - Populate `cxns_vec` with `SG_SIZE` rectangles (with distinct styling, e.g. `Color32::DARK_RED` or `LIGHT_BLUE` / `DARK_BLUE`).
2. Update `SeqGraph::add_ones_val` and add `SeqGraph::add_cxns_val`:
   ```rust
   pub fn add_ones_val(&mut self, ones_fraction: f32) {
       self.add_val(SeqGraphVals::Ones, ones_fraction);
   }

   pub fn add_cxns_val(&mut self, cxns_fraction: f32) {
       self.add_val(SeqGraphVals::Cxns, cxns_fraction);
   }
   ```
3. Update `SeqGraph::add_val`:
   ```rust
   fn add_val(&mut self, sg_vals: SeqGraphVals, fraction: f32) {
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

       let clamped_fraction = fraction.clamp(0.0, 1.0);
       let scaled_height =
           (0.5 + (clamped_fraction - self.zoom.focus) * self.zoom.scale) * SG_HEIGHT;
       let mark_offset = SG_MARK_SIZE / 2.0;
       let vy = self.location().y - (mark_offset + scaled_height);

       let loc = egui::Pos2::new(vx, vy);
       vec_val.last_mut().unwrap().move_to(loc);
   }
   ```
4. Update `SeqGraph::draw_at`:
   - Iterate over `&self.cxns_vec` in addition to `&self.ones_vec` to render all marks.

### File Structure
- `crates/app5/src/canvas/seq_graph.rs`: Target file containing `SeqGraph`, `SeqGraphVals`, and drawing methods.

# Testing

### Validation Approach
Verify fixes using cargo build/check tools on the workspace.

### Key Scenarios
- **Compilation Check**: Run `cargo check --all-targets --workspace` to ensure `app5` and all other workspace crates compile without errors or type mismatches.
- **Method Invocations**: Verify `add_ones_val` and `add_cxns_val` properly modify points in their respective vectors without borrow checker conflicts.

# Delivery Steps

###   Step 1: Fix mutable borrow and target vector in SeqGraph::add_val
Resolve the Rust compiler error E0596 and logical target issues in `crates/app5/src/canvas/seq_graph.rs`.

- Change match arm `SeqGraphVals::Cxns => &self.ones_vec` to `SeqGraphVals::Cxns => &mut self.cxns_vec` in `add_val` so both match branches return `&mut Vec<Rectangle>`.
- Fix the terminal element update on line 167 from `self.ones_vec.last_mut().unwrap().move_to(loc)` to `vec_val.last_mut().unwrap().move_to(loc)` so the correct vector (`ones_vec` or `cxns_vec`) is updated.
- Refactor `add_ones_val` to delegate to `self.add_val(SeqGraphVals::Ones, ones_fraction)` to eliminate code duplication.
- Expose a public `add_cxns_val(&mut self, cxns_fraction: f32)` helper method that calls `self.add_val(SeqGraphVals::Cxns, cxns_fraction)`.

###   Step 2: Initialize cxns_vec and verify workspace compilation
Ensure `cxns_vec` is properly initialized and rendered, and verify that the workspace compiles cleanly.

- Initialize `cxns_vec` in `SeqGraph::new` with `SG_SIZE` rectangles matching graph dimensions and appropriate styling.
- Update `SeqGraph::draw_at` to render `cxns_vec` elements alongside `ones_vec`.
- Execute `cargo check --all-targets --workspace` to confirm that all compile errors in `app5` are resolved and the crate builds successfully.