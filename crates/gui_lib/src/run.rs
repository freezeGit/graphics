
// run.rs
//pub fn run_app() {}

pub fn run_app<A, F>(
    app_name: &str,
    xv: f32,
    yv: f32,
    make_app: F,
) -> Result<(), eframe::Error>
where
    A: eframe::App + 'static,
    F: FnOnce(&eframe::CreationContext<'_>) -> A + 'static,
{
    let native_options = custom_native_options(xv, yv);

    eframe::run_native(
        app_name,
        native_options,
        Box::new(move |cc| Ok(Box::new(make_app(cc)))),
    )
}

// pub fn run_app<A, F>(
//     app_name: &str,
//     xv: f32,
//     yv: f32,
//     make_app: F,
// ) -> Result<(), eframe::Error>
// where
//     A: eframe::App + 'static,
//     F: FnOnce(&eframe::CreationContext<'_>) -> A + 'static,
// {
//     let native_options = custom_native_options(xv, yv);
//
//     eframe::run_native(
//         app_name,
//         native_options,
//         Box::new(move |cc| {
//             cc.egui_ctx.set_visuals(egui::Visuals::light());
//             Ok(Box::new(make_app(cc)))
//         }),
//     )
// }




pub fn custom_native_options(xv: f32, yv: f32) -> eframe::NativeOptions {
    let mut native_options = eframe::NativeOptions::default();
    native_options.viewport = native_options.viewport.with_inner_size(egui::vec2(xv, yv));
    native_options
}