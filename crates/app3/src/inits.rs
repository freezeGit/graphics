//! ## module app_inits. Contains constants and initialization values.

use gui_lib::Color32;
#[allow(unused_imports)]
use gui_lib::LayoutStyle::{NoPanel, SidePanel, TopPanel}; //Any of these styles are valid.

// ------ User customized gui_lib application specific initialization constants --------
pub const APP_NAME: &str = "App using gui_lib"; // Application name.
pub const XWVP: f32 = 1200.0; // Width of viewport in pixels.
pub const YHVP: f32 = 800.0; // Height of viewport in pixels.

// Layout styles: TopPanel, SidePanel, NoPanel
pub const LAYOUT_STYLE: gui_lib::LayoutStyle = TopPanel;

// Background colors: BKG_DEFAULT, BKG_WINDOWS, or any Color32
pub const BACKGROUND_COLOR: Color32 = gui_lib::BKG_DEFAULT;

// ------ User customized timer initialization constants --------

// `INTERVAL`: Time between simulation steps in seconds
//pub const INTERVAL: f64 = 0.5;
pub const INTERVAL: f64 = 0.05;
// `BATCH_SIZE`: Number of world advances to perform in a single simulation step
// during fast-forward of the simulation.
//pub const BATCH_SIZE: u32 = 1;
pub const BATCH_SIZE: u32 = 100;
// `SMOOTH_ANIMATION`: If true, the simulation will request repaint at 16ms intervals.
// This may result in a smoother animation, but may also cause performance issues
// because of extra refresh requests. If false, the simulation will request repaint
// at intervals determined by INTERVAL.
//pub const SMOOTH_ANIMATION: bool = true;
pub const SMOOTH_ANIMATION: bool = false;
// 'INITIAL_RULE': Initial rule number for the simulation.
// Valid values are 0 to 15.

// ------ User customized simulation initialization constants --------

//pub const INITIAL_RULE: u8 = 15; // must be 0 to 15 inclusive
pub const INITIAL_RULE: u8 = 5; // must be 0 to 15 inclusive
pub const INITIAL_BITS_NUM: usize = 6000; // 6000 to exactly fill a 100 * 60 grid.
pub const INITIAL_ONES: usize = 0; // Must be <= INITIAL_BITS_NUM.

pub const INITIAL_SEQ_DISCARD: usize = 45000;
pub const INITIAL_SEQ_LENGTH: usize = 1000;
