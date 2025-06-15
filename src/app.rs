use crate::pulse::Circle;
use egui::{CentralPanel, Color32, Context, Label, Rect, Visuals, include_image};

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
    fn clear_color(&self, _: &Visuals) -> [f32; 4] {
        [0.0, 0.0, 0.0, 0.0]
    }

    fn update(&mut self, ctx: &Context, _: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            ui.image(include_image!("../assets/sample_cv.svg"));

            if cfg!(debug_assertions) {
                let rect = Rect::from_min_max((120., 265.).into(), (865., 455.).into());
                ui.painter().debug_rect(rect, Color32::RED, "");
                ui.put(
                    Rect::from_min_max((0., 0.).into(), (100., 50.).into()),
                    Label::new(
                        ui.ctx()
                            .pointer_latest_pos()
                            .unwrap_or(egui::Pos2 { x: -1., y: -1. })
                            .to_string(),
                    ),
                );
            }

            if ui.add(&mut self.circles).contains_pointer() {
                self.circles.start_animation();
            } else {
                self.circles.stop_animation();
            };
        });
    }
}
