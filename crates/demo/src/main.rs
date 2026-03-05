// use graphics::demo::run_the_app;
//
// ///Demonstrate module gui_lib code using module demo
// fn main() -> Result<(), eframe::Error> {
//     run_the_app()
// }

// mod demo;
//
// use demo::run_the_app;
//
// fn main() {
//     run_the_app();
// }

mod demo;

fn main() {
    demo::run_the_app();
}