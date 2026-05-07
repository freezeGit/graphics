//! # Module world_gl contains the World trait
// world_gl.rs

/// A trait that represents a "world" or a system that can be advanced
/// or updated over time.
///
/// This is typically used in simulation or game
/// development contexts, but it can also support other types of systems
/// that exhibit temporal progression.
///
/// # Default Behavior
/// The default implementation of the `advance` function does nothing.
/// This allows non-simulation applications or systems that do not require
/// temporal updates to use this trait without implementing specific logic.

/// # Notes
/// - Implementors of this trait can provide their own logic for the `advance` method
///   to customize how the world evolves during each update.
/// - If no custom behavior is needed, the default `advance` implementation can be used.
pub trait World {
    fn advance(&mut self) {
        // Default: do nothing.
        // Non-simulation apps can use this as-is.
    }
}

// Back from the future
