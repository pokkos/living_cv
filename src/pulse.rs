use egui::{
    Color32, Id, Pos2, Rect, Response, Sense, Shape, Stroke, Ui, Widget, epaint::CircleShape,
};

const MIN_RADIUS: f32 = 1.;
const MAX_RADIUS: f32 = 10.;
const ANIMATION_TIME: f32 = 1.5;

pub struct Circle {
    id: Id,
    radius: f32,
    is_animated: bool,
    was_animated: bool,
    hover_rect: Rect,
    is_popup_visible: bool,
    position: Pos2,
}

impl Circle {
    pub fn new(rect: Rect, id: String) -> Self {
        Self {
            id: Id::from(id),
            hover_rect: rect,
            radius: MIN_RADIUS,
            is_animated: true,
            was_animated: true,
            is_popup_visible: false,
            position: Pos2 { x: 0., y: 0. },
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

    pub fn get_position(&self) -> Pos2 {
        self.position
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

impl Widget for &mut Circle {
    fn ui(self, ui: &mut Ui) -> Response {
        // set the position for the pulsating circle
        self.position = self.hover_rect.right_center();
        self.position.x -= 2. * MAX_RADIUS;

        let stroke = Stroke::new(3., Color32::MAGENTA);

        // allocate the hover rectangle that enables the animation and interaction
        let resp = ui.allocate_rect(self.hover_rect, Sense::click());

        // fix the animation flickering by setting the previous radius when the state changes
        if !self.is_animated && self.was_animated {
            ui.ctx().clear_animations();
            self.radius = ui.ctx().animate_value_with_time(self.id, self.radius, 0.);
            ui.ctx().request_repaint();
            self.was_animated = false;
        }

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
            self.was_animated = true;
        }

        // draw the rectangle when hovered
        if resp.contains_pointer() {
            ui.painter().add(Shape::rect_stroke(
                self.hover_rect,
                5.,
                egui::Stroke::new(2., Color32::RED),
                egui::StrokeKind::Inside,
            ));
        }

        // actually draw the circle if it's in view
        if ui.is_rect_visible(self.hover_rect) {
            ui.painter().add(CircleShape {
                center: self.position,
                radius: self.radius,
                fill: Color32::TRANSPARENT,
                stroke,
            });
        }

        resp
    }
}
