use std::collections::HashMap;

use egui::{Hyperlink, ImageSource, Vec2};

pub struct Popup {
    data: Vec<(String, String)>,
    panel_size: Vec2,
    images: HashMap<String, ImageSource<'static>>,
}

impl Popup {
    pub fn new(label: &String, panel_size: Vec2) -> Option<Self> {
        let file = include_str!("../popup.toml");
        let readin = file.parse::<toml::Table>().ok()?;

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

        // hardcode the image paths for now
        let mut images: HashMap<String, ImageSource<'static>> = HashMap::new();
        images.insert(
            String::from("header.jpg"),
            egui::include_image!("../assets/images/header.jpg"),
        );

        if data.is_empty() {
            None
        } else {
            Some(Self {
                data,
                panel_size,
                images,
            })
        }
    }

    pub fn data(&self) -> &Vec<(String, String)> {
        &self.data
    }
}

impl Popup {
    pub fn show(&mut self, ui: &mut egui::Ui) -> egui::ModalResponse<()> {
        let framestyle = egui::containers::Frame {
            inner_margin: egui::epaint::Margin {
                left: 2,
                right: 2,
                top: 2,
                bottom: 2,
            },
            outer_margin: egui::epaint::Margin {
                left: 0,
                right: 0,
                top: 0,
                bottom: 0,
            },
            corner_radius: egui::CornerRadius {
                nw: 8,
                ne: 8,
                sw: 8,
                se: 8,
            },
            shadow: eframe::epaint::Shadow {
                color: egui::Color32::from_hex("#000000aa").unwrap(),
                offset: [10, 10],
                blur: 20,
                spread: 10,
            },
            fill: egui::Color32::from_hex("#ffffff").unwrap(),
            stroke: egui::Stroke::new(2.0, egui::Color32::from_hex("#266590").unwrap()),
        };
        egui::Modal::new(egui::Id::new("modal"))
            .frame(framestyle)
            .show(ui.ctx(), |ui| {
                for (key, value) in &self.data {
                    match key.as_str() {
                        "image" => {
                            if let Some(img) = self.images.get(value) {
                                ui.add(
                                    egui::Image::new(img.clone())
                                        .corner_radius(5)
                                        .maintain_aspect_ratio(true)
                                        .max_width(&self.panel_size.x * 0.7)
                                        .max_height(&self.panel_size.y * 0.7)
                                        .fit_to_fraction(Vec2::from((2.0, 2.0))),
                                );
                            };
                        }
                        "label" => {
                            ui.label(value);
                        }
                        "link" => {
                            ui.add(
                                Hyperlink::from_label_and_url(value, value).open_in_new_tab(true),
                            );
                        }
                        _ => (),
                    }
                }
            })
    }
}
