use egui::{Vec2, Widget};
use toml;

pub struct Popup {
    data: Vec<(String, String)>,
    panel_size: Vec2,
}

impl Popup {
    pub fn new(label: &String, panel_size: Vec2) -> Self {
        let file = include_str!("../popup.toml");
        let readin = file
            .parse::<toml::Table>()
            .expect("No popup toml file found.");

        let mut data = Vec::default();
        if let Some(items) = readin.get(label) {
            if let Some(item) = items.get("items") {
                for val in item.as_array().unwrap() {
                    for (key, v) in val.as_table().unwrap() {
                        data.push((key.as_str().to_string(), v.as_str().unwrap().to_string()));
                    }
                }
            }
        }

        Self { data, panel_size }
    }

    pub fn data(&self) -> &Vec<(String, String)> {
        &self.data
    }
}

// impl Widget for &mut Popup {
//     fn ui(self, ui: &mut egui::Ui) -> egui::Response {
//         let modal = egui::Modal::new(egui::Id::new("modal")).show(ui.ctx(), |ui| {
//             for (key, value) in &self.data {
//                 match key.as_str() {
//                     "image" => {
//                         let my_str = value.as_str();
//                         ui.add(
//                             egui::Image::new(format!("file://{my_str}"))
//                                 .corner_radius(5)
//                                 .maintain_aspect_ratio(true)
//                                 .max_width(&self.panel_size.x * 0.7)
//                                 .max_height(&self.panel_size.y * 0.7)
//                                 .fit_to_fraction(Vec2::from((2.0, 2.0))),
//                         );
//                     }
//                     "label" => {
//                         ui.label(value);
//                     }
//                     "link" => {
//                         ui.hyperlink_to(value, value);
//                     }
//                     _ => (),
//                 };
//             }
//         });

//         // if modal.should_close() {
//         //     area.hide_popup();
//         // }

//         modal.response
//     }
// }
