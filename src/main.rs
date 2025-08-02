
use graphics::*;
fn main() -> Result<(), eframe::Error> {
    //let native_options = eframe::NativeOptions::default();  // If defults are good enough
    let mut native_options = eframe::NativeOptions::default();
    native_options.viewport = native_options.viewport.with_inner_size(egui::vec2(1200.0, 800.0));
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

//Want to dig into how eframe::App works behind the scenes? Or are you good for now?