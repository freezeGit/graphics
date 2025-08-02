

// use graphics::{custom_light_visuals, MyApp};
use graphics::*;
fn main() -> Result<(), eframe::Error> {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "GUI Draw Example",
        native_options,
        Box::new(|cc| {
            cc.egui_ctx.set_visuals(custom_light_visuals());
            let app: Box<dyn eframe::App> = Box::new(MyApp::new());
            Ok(app)
        }),
    )
}