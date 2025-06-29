use egui::{
    Color32, Id, Pos2, Rect, Response, Sense, Shape, Stroke, Ui, Widget, epaint::CircleShape,
};

const MIN_RADIUS: f32 = 1.;
const MAX_RADIUS: f32 = 10.;
const ANIMATION_TIME: f32 = 1.5;

pub struct Overlay {
    hover_rect: Rect,
    id: Id,
    is_popup_visible: bool,
    position: Pos2,
    radius: f32,
}

impl Overlay {
    pub fn new(rect: Rect, id: String) -> Self {
        Self {
            id: Id::from(id),
            hover_rect: rect,
            radius: MIN_RADIUS,
            is_popup_visible: false,
            position: Pos2::default(),
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
        // set the position for the pulsating circle
        self.position = self.hover_rect.right_center();
        self.position.x -= 2. * MAX_RADIUS;

        let stroke = Stroke::new(3., Color32::from_hex("#09a7cb").unwrap());

        // allocate the hover rectangle that enables the animation and interaction
        let resp = ui.allocate_rect(self.hover_rect, Sense::click());

        // draw the rectangle when hovered and animate the circle
        if resp.contains_pointer() {
            ui.painter().add(Shape::rect_stroke(
                self.hover_rect,
                5.,
                egui::Stroke::new(2., Color32::from_hex("#09a7cb").unwrap()),
                egui::StrokeKind::Inside,
            ));

            if self.radius >= MAX_RADIUS {
                self.radius = ui.ctx().animate_value_with_time(self.id, MIN_RADIUS, 0.);
            }

            self.radius = ui
                .ctx()
                .animate_value_with_time(self.id, MAX_RADIUS, ANIMATION_TIME);
        } else {
            self.radius =
                ui.ctx()
                    .animate_value_with_time(self.id, MIN_RADIUS, ANIMATION_TIME / 4.);
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

        ui.ctx().request_repaint();

        resp
    }
}
