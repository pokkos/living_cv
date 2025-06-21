use crate::pulse::Circle;
use egui::{
    CentralPanel, Color32, Context, Frame, Id, Label, Modal, Pos2, Rect, Ui, Vec2, Visuals,
    include_image,
};

pub struct App {
    circles: Vec<Circle>,
}

impl App {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        cc.egui_ctx.set_visuals(Visuals {
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
                debug_output(ui, &img_resp.rect);
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
                    Modal::new(Id::new("modal"))
                        .backdrop_color(Color32::from_hex("#aaddee55").unwrap())
                        .show(ui.ctx(), |ui| {
                            ui.visuals_mut().faint_bg_color = Color32::RED;
                            Frame::canvas(ui.style())
                                .fill(Color32::from_hex("#ccdde9").unwrap())
                                .show(ui, |ui| ui.allocate_space(Vec2 { x: 400., y: 300. }));
                        });
                }
            }
        });
    }
}

fn debug_output(ui: &mut Ui, rect: &Rect) {
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
        Label::new(rect.to_string()),
    );
}
