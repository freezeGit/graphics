---
sessionId: session-261022-163548-v05e
---

# Requirements

### Overview & Goals
Fix the compiler error `unexpected closing delimiter: '}'` in `crates/app4/src/canvas.rs` caused by a mismatched closing brace in `TheCanvas::update`, and ensure rule text and sample size are formatted and updated correctly.

### Scope
- **In Scope**:
  - `crates/app4/src/canvas.rs`: Correct brace matching in `TheCanvas::update` and clean up the `txt` rule display logic.
- **Out of Scope**:
  - Unrelated files and crates in the workspace.

### Functional Requirements
- `TheCanvas::update` must parse and compile cleanly in Rust.
- If `world.bits_rule.random()` is true, `stxt_rule` displays `"Rule: R"`.
- If `world.bits_rule.random()` is false, `stxt_rule` displays `"Rule: <number>"`.
- `stxt_sample_size` continues to display `"Sample size: <sample_size>"`.
- `cargo check -p app4` must succeed with zero errors.

# Technical Design

### Current Implementation
In `crates/app4/src/canvas.rs` (lines 122–140):
```rust
    pub fn update(&mut self, world: &mut TheWorld) {
        if world.bits_rule.random() {
            let txt = "R";
        } else {
            let txt = world.bits_rule.number();
        }

        self.view_handles
            .stxt_rule
            .borrow_mut()
            .set_text(format!("Rule: {}", world.bits_rule.number()));
        }

        self.view_handles
            .stxt_sample_size
            .borrow_mut()
            .set_text(format!("Sample size: {}", world.sample_size));
    }
} // end of impl TheCanvas
```

Two bugs are present:
1. An extra closing brace `}` on line 133 prematurely ends `pub fn update(...)`. The remaining statements (`stxt_sample_size...`) end up outside any method body, causing line 140's closing brace to be flagged as `unexpected closing delimiter: '}'`.
2. The `let txt` variables inside `if` / `else` are scoped and discarded, while `stxt_rule` unconditionally prints `world.bits_rule.number()`.

### Key Decisions
- Use an `if/else` expression returning a formatted string or `&str` / `String` for `txt`, then interpolate `txt` into `"Rule: {}"`.
- Remove the spurious closing brace on line 133 so `update` encloses both `stxt_rule` and `stxt_sample_size` updates.

### Proposed Changes
Update `TheCanvas::update` in `crates/app4/src/canvas.rs` to:
```rust
    pub fn update(&mut self, world: &mut TheWorld) {
        let txt = if world.bits_rule.random() {
            "R".to_string()
        } else {
            world.bits_rule.number().to_string()
        };

        self.view_handles
            .stxt_rule
            .borrow_mut()
            .set_text(format!("Rule: {}", txt));

        self.view_handles
            .stxt_sample_size
            .borrow_mut()
            .set_text(format!("Sample size: {}", world.sample_size));
    }
```

# Testing

### Validation Approach
- Run `cargo check -p app4` to ensure syntax errors are resolved and the crate compiles cleanly.
- Run `cargo check --workspace` to ensure all crates in the workspace build cleanly without regressions.

### Key Scenarios
1. **Compilation Check**: `cargo check -p app4` produces exit code 0.
2. **Brace Nesting**: All methods within `impl TheCanvas` are properly enclosed by braces up to the `impl` closing brace.

# Delivery Steps

###   Step 1: Fix syntax and rule display in TheCanvas::update
Fix syntax errors and rule text formatting in `TheCanvas::update` in `crates/app4/src/canvas.rs`.

- Remove the extra closing brace `}` after `stxt_rule` text assignment.
- Properly bind `txt` from the `if world.bits_rule.random() { ... } else { ... }` expression.
- Format `stxt_rule` with `txt` (showing `"R"` for random rules or the rule number for fixed rules).
- Keep `stxt_sample_size` inside `TheCanvas::update` with matching delimiters for both `update` and `impl TheCanvas`.

###   Step 2: Verify compilation of app4 crate
Verify that `app4` compiles cleanly without delimiter or syntax errors.

- Run `cargo check -p app4` to confirm compilation errors are resolved.
- Run `cargo test -p app4` (or workspace check) to ensure no regressions.