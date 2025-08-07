use graphics::*;

///Demonstrate module gui_lib code using module demo
fn main() -> Result<(), eframe::Error> {
    run_demo()
}

//--------------------------------------------------
// fn main() -> Result<(), eframe::Error> {
//     let mut native_options = eframe::NativeOptions::default();
//     native_options.viewport = native_options.viewport.with_inner_size(vec2(1200.0, 800.0));
//     eframe::run_native(
//         "GUI Draw Example",
//         native_options,
//         Box::new(|cc| {
//             cc.egui_ctx.set_visuals(custom_light_visuals()); //custom_light_visuals() lib.rs
//             //cc.egui_ctx.set_visuals(eframe::egui::Visuals::light()); //light theme
//             //cc.egui_ctx.set_visuals(eframe::egui::Visuals::dark()); //dark theme (default)
//             let app: Box<dyn eframe::App> = Box::new(DemoApp::new());
//             Ok(app)
//         }),
//     )
// }