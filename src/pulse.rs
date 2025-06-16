use egui::epaint::CircleShape;
use egui::{Color32, Id, Rect, Response, Sense, Stroke, Ui, Widget};

const MIN_RADIUS: f32 = 1.;
const MAX_RADIUS: f32 = 10.;
const ANIMATION_TIME: f32 = 1.5;

pub struct Circle {
    id: Id,
    radius: f32,
    is_animated: bool,
    hover_rect: Rect,
}

impl Circle {
    pub fn new(rect: Rect, id: Id) -> Self {
        Self {
            id,
            hover_rect: rect,
            radius: MIN_RADIUS,
            is_animated: true,
        }
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

    pub fn get_rect(&self) -> Rect {
        self.hover_rect
    }
}

impl Widget for &mut Circle {
    fn ui(self, ui: &mut Ui) -> Response {
        // set the position for the pulsating circle
        let mut position = self.hover_rect.right_center();
        position.x -= 2. * MAX_RADIUS;

        let stroke = Stroke::new(3., Color32::MAGENTA);

        // allocate the hover rectangle that enables the animation and interaction
        let resp = ui.allocate_rect(self.hover_rect, Sense::click());

        // if the animation is stopped or the maximum radius is reached, reset the radius to the min value
        if self.radius == MAX_RADIUS || !self.is_animated {
            self.radius = ui.ctx().animate_value_with_time(self.id, MIN_RADIUS, 0.);
            ui.ctx().request_repaint();
        }

        // animate the radius of the circle
        if self.is_animated {
            self.radius = ui
                .ctx()
                .animate_value_with_time(self.id, MAX_RADIUS, ANIMATION_TIME);
            ui.ctx().request_repaint();
        }

        // actually draw the circle if it's in view
        if ui.is_rect_visible(self.hover_rect) {
            ui.painter().add(CircleShape {
                center: position,
                radius: self.radius,
                fill: Color32::TRANSPARENT,
                stroke,
            });
        }

        resp
    }
}
