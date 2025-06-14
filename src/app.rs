use crate::pulse::Circle;
use egui::{CentralPanel, Label, Rect, include_image};

#[derive(Default)]
pub struct App {
    circles: Circle,
}

impl App {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        cc.egui_ctx.set_visuals(egui::Visuals {
            dark_mode: false,
            panel_fill: egui::Color32::TRANSPARENT,
            ..Default::default()
        });

        Self::default()
    }
}

impl eframe::App for App {
    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        [0.0, 0.0, 0.0, 0.0]
    }
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            ui.image(include_image!("../assets/sample_cv.svg"));

            if ui.add(&mut self.circles).contains_pointer() {
                self.circles.start_animation();
            } else {
                self.circles.stop_animation();
            };
        });
    }
}
