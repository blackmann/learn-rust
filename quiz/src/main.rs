mod quiz;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
    eframe::run_native("Queeez", options, Box::new(|ctx| Box::new(App::new())))
}

struct App {}

impl App {
    fn new() -> Self {
        Self {}
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("1/10");
            ui.label("What is the capital of Nigeria?");

            ui.horizontal(|ui| {
                ui.button("True");
                ui.button("False");
            });

            ui.horizontal(|ui| {
                ui.button("Previous");
                ui.button("Next");
            });
        });
    }
}
