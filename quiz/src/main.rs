mod quiz;

fn main() -> Result<(), eframe::Error> {
    let viewport = egui::ViewportBuilder::default().with_inner_size(egui::Vec2::new(300.0, 400.0));

    let options = eframe::NativeOptions{
      viewport,
      // This is a common pattern in Rust. See below
      ..Default::default()
    };

    eframe::run_native("Queeez", options, Box::new(|ctx| Box::new(App::new())))
}

struct App {
    quiz: quiz::Quiz,
}

impl App {
    fn new() -> Self {
        let quiz = quiz::Quiz::sample();
        Self { quiz }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let current_index = self.quiz.current_index + 1;
            let count = self.quiz.questions.len();

            // welcome to string formatting in Rust
            ui.label(format!("{current_index}/{count}"));

            let current_question = self.quiz.current_question();
            ui.label(&current_question.title);

            ui.horizontal(|ui| {
                if ui.button("True").clicked() {
                    self.quiz.answer(true);
                }
                if ui.button("False").clicked() {
                    self.quiz.answer(false);
                }
            });

            ui.columns(2, |columns| {
                columns[0].allocate_ui_with_layout(
                  egui::Vec2 { x: 120.0, y: 40.0 },
                  egui::Layout::left_to_right(egui::Align::Center),
                  |ui| {
                    if ui.button("previous").clicked() {
                        self.quiz.previous();
                    }
                  },
                );

                columns[1].allocate_ui_with_layout(
                  egui::Vec2 { x: 120.0, y: 40.0 },
                  egui::Layout::right_to_left(egui::Align::Center),
                  |ui| {
                    if ui.button("next").clicked() {
                        self.quiz.next_question();
                    }
                  },
                );
              })
        });
    }
}
