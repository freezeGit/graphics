use eframe::egui;

pub trait Draw {
    fn draw(&self, ui: &mut egui::Ui);
}

pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self, ui: &mut egui::Ui) {
        for component in &self.components {
            component.draw(ui);
        }
    }
}

pub struct Button {
    pub width: f32,
    pub height: f32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self, ui: &mut egui::Ui) {
        let size = egui::vec2(self.width, self.height);
        ui.add_sized(size, egui::Button::new(&self.label));
    }
}

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
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            self.screen.run(ui);
        });
    }
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "GUI Draw Example",
        options,
        Box::new(|_cc| Ok::<Box<dyn eframe::App>, _>(Box::new(MyApp::new()))),
    )
}