use egui::epaint::CircleShape;
use egui::{Color32, Id, Pos2, Rect, Response, Sense, Stroke, Ui, Widget};

const MIN_RADIUS: f32 = 1.;
const MAX_RADIUS: f32 = 10.;
const ANIMATION_TIME: f32 = 1.5;

pub struct Circle {
    id: Id,
    radius: f32,
    is_animated: bool,
}

impl Default for Circle {
    fn default() -> Self {
        Self {
            id: Id::new("pulsating_circle"),
            radius: MIN_RADIUS,
            is_animated: true,
        }
    }
}

impl Circle {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn start_animation(&mut self) {
        self.is_animated = true;
    }

    pub fn stop_animation(&mut self) {
        self.is_animated = false;
    }

    pub fn toggle_animation(&mut self) {
        self.is_animated = !self.is_animated;
    }
}

impl Widget for &mut Circle {
    fn ui(self, ui: &mut Ui) -> Response {
        let position = Pos2::new(865. - 2. * MAX_RADIUS, 360.);
        let rect_hover_field = Rect::from_min_max((120., 265.).into(), (865., 455.).into());
        let stroke = Stroke::new(3., Color32::MAGENTA);

        let resp = ui.allocate_rect(rect_hover_field, Sense::click());

        if self.radius == MAX_RADIUS || !self.is_animated {
            self.radius = MIN_RADIUS;

            ui.ctx().clear_animations();
            self.radius = ui.ctx().animate_value_with_time(self.id, self.radius, 0.);
            ui.ctx().request_repaint();
        }

        if self.is_animated {
            self.radius = ui
                .ctx()
                .animate_value_with_time(self.id, MAX_RADIUS, ANIMATION_TIME);
            ui.ctx().request_repaint();
        }

        if ui.is_rect_visible(rect_hover_field) {
            let painter = ui.painter();

            painter.add(CircleShape {
                center: position,
                radius: self.radius,
                fill: Color32::TRANSPARENT,
                stroke,
            });
        }

        resp
    }
}
