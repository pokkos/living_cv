use crate::pulse::Circle;
use egui::{
    CentralPanel, Color32, Context, Label, Pos2, Rect, Vec2, Visuals, Window, include_image,
};

pub struct App {
    circles: Vec<Circle>,
}

impl App {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        cc.egui_ctx.set_visuals(egui::Visuals {
            dark_mode: false,
            panel_fill: Color32::TRANSPARENT,
            ..Default::default()
        });

        // temporarily hard-coded coordinates
        let circles = vec![
            Circle::new(
                Rect::from_min_max((120., 265.).into(), (865., 455.).into()),
                "pulse_1".into(),
            ),
            Circle::new(
                Rect::from_min_max((120., 500.).into(), (865., 690.).into()),
                "pulse_2".into(),
            ),
        ];

        Self { circles }
    }
}

impl eframe::App for App {
    fn clear_color(&self, _: &Visuals) -> [f32; 4] {
        [0.0, 0.0, 0.0, 0.0]
    }

    fn update(&mut self, ctx: &Context, _: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            // draw the background image (cv)
            let img_resp = ui.image(include_image!("../assets/sample_cv.svg"));

            // debug helpers
            if cfg!(debug_assertions) {
                ui.put(
                    Rect::from_min_max((0., 0.).into(), (100., 50.).into()),
                    Label::new(
                        ui.ctx()
                            .pointer_latest_pos()
                            .unwrap_or(Pos2 { x: -1., y: -1. })
                            .to_string(),
                    ),
                );
                ui.put(
                    Rect::from_min_max((0., 50.).into(), (200., 100.).into()),
                    Label::new(img_resp.rect.to_string()),
                );
            }

            // check for hovering areas and start the relevant animation
            for pulse in self.circles.iter_mut() {
                let resp = ui.add(&mut *pulse);

                // animate the circle that is in the hovered region
                if resp.contains_pointer() {
                    pulse.start_animation();
                } else {
                    pulse.stop_animation();
                };

                // open the window at the position where the circle is
                if resp.clicked() && !pulse.is_popup_visible() {
                    pulse.show_popup();
                };

                if resp.clicked_elsewhere() {
                    pulse.hide_popup();
                }

                if pulse.is_popup_visible() {
                    let mut is_open = true;
                    let win = Window::new("Info")
                        .fixed_pos(pulse.get_position())
                        .fixed_size(Vec2::new(300., 300.))
                        .resizable(false)
                        .collapsible(false)
                        .open(&mut is_open);
                    win.show(ui.ctx(), |ui| ui.allocate_space(ui.available_size()));
                }
            }
        });
    }
}
