
// Working code before adding modules
// mod demo;
//
// fn main() {
//     demo::run_the_app();
// }

//new stuff
// ----------------------------
mod ids;
mod world;
mod canvas;
mod app;
mod demo; // optional. Shrink and delete finally

fn main() -> Result<(), eframe::Error> {
    demo::run_the_app()
    // or app::run_the_app() if you move it there
}