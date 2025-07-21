use egui::{Color32, Rect, Response, Sense, Shape, Ui, Vec2, Widget};

use crate::popup::Popup;

pub struct Overlay {
    hover_rect: Rect,
    is_popup_visible: bool,
    label: String,
    popup: Option<Popup>,
}

impl Overlay {
    pub fn new(rect: Rect, label: String, panel_size: Vec2) -> Self {
        let popup = Popup::new(&label, panel_size);

        Self {
            hover_rect: rect,
            is_popup_visible: false,
            label,
            popup,
        }
    }

    pub fn has_popup(&self) -> bool {
        if self.popup.is_some() { true } else { false }
    }

    pub fn show_popup(&mut self) {
        if self.popup.is_some() {
            self.is_popup_visible = true;
        };
    }

    pub fn hide_popup(&mut self) {
        self.is_popup_visible = false;
    }

    pub fn is_popup_visible(&self) -> bool {
        self.is_popup_visible
    }

    pub fn popup(&mut self) -> Option<&mut Popup> {
        self.popup.as_mut()
    }

    pub fn label(&self) -> String {
        self.label.clone()
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
        }

        // open the modal window
        if resp.clicked() {
            self.show_popup();
        }

        if self.is_popup_visible() {
            let modal = self.popup().unwrap().show(ui);
            if modal.should_close() {
                self.hide_popup();
            }
        }

        resp
    }
}
