//! ## module app_inits. Contains constants and initialization values.

use gui_lib::Color32;
#[allow(unused_imports)]
use gui_lib::LayoutStyle::{TopPanel, SidePanel, NoPanel};

// ------ User customized gui_lib application specific initialization constants --------
pub const APP_NAME: &str = "App using gui_lib"; // Application name.
pub const XWVP: f32 = 1200.0; // Width of viewport in pixels.
pub const YHVP: f32 = 800.0; // Height of viewport in pixels.

// Layout styles: TopPanel, SidePanel, NoPanel
pub const LAYOUT_STYLE: gui_lib::LayoutStyle = TopPanel;

// Background colors: BKG_DEFAULT, BKG_WINDOWS, or any Color32
pub const BACKGROUND_COLOR: Color32 = gui_lib::BKG_DEFAULT;

// ------ User customized simulation initialization constants --------

// `INTERVAL`: Time between simulation steps in seconds
pub const INTERVAL: f64 = 0.5;
// `BATCH_SIZE`: Number of world advances to perform in a single simulation step
// during fast-forward of the simulation.
pub const BATCH_SIZE: u32 = 1;
// `SMOOTH_ANIMATION`: If true, the simulation will request repaint at 16ms intervals.
// This may result in a smoother animation, but may also cause performance issues
// because of extra refresh requests. If false, the simulation will request repaint
// at intervals determined by INTERVAL.
pub const SMOOTH_ANIMATION: bool = true;
