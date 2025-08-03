
// lib.rs

// ------------------------------
// Reusable module
// ------------------------------
mod gui_lib {
    pub use eframe::egui::{Ui, Button as EguiButton, vec2, Visuals, Color32};

    /// Set to custom light style, with Windows 10 light gray background
    pub fn custom_light_visuals() -> Visuals {
        let mut visuals = Visuals::light();
        let win10_gray = Color32::from_rgb(240, 240, 240);

        visuals.extreme_bg_color = win10_gray;
        visuals.window_fill = win10_gray;
        visuals.panel_fill = win10_gray;
        visuals.widgets.inactive.bg_fill = win10_gray;
        visuals.override_text_color = Some(Color32::BLACK);

        visuals
    }
    pub trait Draw {
        fn draw(&self, ui: &mut Ui);
    }
    pub struct Button {
        pub width: f32,
        pub height: f32,
        pub label: String,
    }

    impl Draw for Button {
        fn draw(&self, ui: &mut Ui) {
            let size = vec2(self.width, self.height);
            ui.add_sized(size, EguiButton::new(&self.label));
        }
    }
    pub struct Screen {
        pub components: Vec<Box<dyn Draw>>,
    }

    impl Screen {
        // pub fn run(&self, ui: &mut egui::Ui) {
        pub fn run(&self, ui: &mut Ui) {
            for component in &self.components {
                component.draw(ui);
            }
        }
    }
}

// ------------------------------
// App-specific module
// ------------------------------
mod app {
    // use super::gui_lib::{Draw, custom_light_visuals, Screen, Button};
    use super::gui_lib::{Screen, Button};
    // use eframe::egui::{Context, CentralPanel, Ui};
    use eframe::egui::{Context, CentralPanel};
    
    pub struct MyApp {
        screen: Screen,
    }

    impl MyApp {
        pub fn new() -> Self {
            Self {
                screen: Screen {
                    components: vec![Box::new(Button {
                        width: 120.0,
                        height: 40.0,
                        label: "Click Me!".to_string(),
                    })],
                },
            }
        }
    }

    impl eframe::App for MyApp {
        fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
            CentralPanel::default().show(ctx, |ui| {
                self.screen.run(ui);
            });
        }
    }
}

// Optionally expose parts of the modules publicly
pub use gui_lib::{Draw, custom_light_visuals};
pub use app::MyApp;
pub use eframe::egui::vec2;  // Add this line
