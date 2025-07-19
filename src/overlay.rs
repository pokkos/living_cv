use egui::{Color32, Rect, Response, Sense, Shape, Ui, Widget};

pub struct Overlay {
    hover_rect: Rect,
    is_popup_visible: bool,
    label: String,
}

impl Overlay {
    pub fn new(rect: Rect, label: String) -> Self {
        Self {
            hover_rect: rect,
            is_popup_visible: false,
            label,
        }
    }

    pub fn show_popup(&mut self) {
        self.is_popup_visible = true;
    }

    pub fn hide_popup(&mut self) {
        self.is_popup_visible = false;
    }

    pub fn is_popup_visible(&self) -> bool {
        self.is_popup_visible
    }
}

impl Widget for &mut Overlay {
    fn ui(self, ui: &mut Ui) -> Response {
        // allocate the hover rectangle that enables the interaction
        let resp = ui.allocate_rect(self.hover_rect, Sense::click());

        // always draw the rectangle, but filled and with thicker stroke when hovered
        if ui.is_rect_visible(self.hover_rect) {
            let mut stroke_width = 0.5;
            let mut stroke_color = Color32::from_hex("#aaaaaa").unwrap_or(Color32::LIGHT_BLUE);
            let corner_radius = 5.;

            if resp.contains_pointer() {
                ui.painter().add(Shape::rect_filled(
                    self.hover_rect,
                    corner_radius,
                    Color32::from_hex("#09a7cb11").unwrap_or(Color32::LIGHT_BLUE),
                ));

                // change the stroke width for the hovered rect
                stroke_width = 2.;
                stroke_color = Color32::from_hex("#09a7cb").unwrap_or(Color32::LIGHT_BLUE);
            }

            ui.painter().add(Shape::rect_stroke(
                self.hover_rect,
                corner_radius,
                egui::Stroke::new(stroke_width, stroke_color),
                egui::StrokeKind::Inside,
            ));

            if self.is_popup_visible() {
                egui::Modal::new(egui::Id::new("modal"))
                    .backdrop_color(Color32::from_hex("#aaddee55").unwrap())
                    .show(ui.ctx(), |ui| {
                        ui.visuals_mut().faint_bg_color = Color32::RED;
                        egui::Frame::canvas(ui.style())
                            .fill(Color32::from_hex("#ccdde9").unwrap())
                            .show(ui, |ui| {
                                // ui.allocate_space(egui::Vec2 { x: 400., y: 300. });
                                ui.label(&self.label);
                            });
                    });
            }
        }

        resp
    }
}
