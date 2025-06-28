use crate::{document::TypstWorld, pulse::Circle};
use egui::{
    CentralPanel, Color32, ColorImage, Context, Frame, Id, Label, Modal, Pos2, Rect, Shape, Ui,
    Vec2, Visuals,
};
use typst::layout::{FrameItem, PagedDocument};

pub struct App {
    circles: Vec<Circle>,
    texture: egui::TextureHandle,
}

impl App {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        cc.egui_ctx.set_visuals(Visuals {
            dark_mode: false,
            panel_fill: Color32::WHITE,
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

        Self {
            circles,
            texture: cc.egui_ctx.load_texture(
                "background",
                egui::ColorImage::example(),
                egui::TextureOptions::NEAREST,
            ),
        }
    }
}

fn document_info(ui: &Ui) -> (u32, u32, Vec<u8>) {
    let content = std::include_str!("../assets/cv.typ").to_string();

    let world = TypstWorld::new(content);
    let document: PagedDocument = typst::compile(&world).output.expect("Compile error.");
    let mut item_positions: Vec<(Pos2, Pos2)> = Vec::new();

    let panel_height = ui.available_height();
    let panel_width = ui.available_width();
    let doc_height = document.pages[0].frame.size().y.to_pt() as f32;
    let doc_width = document.pages[0].frame.size().x.to_pt() as f32;
    let panel_ratio = panel_width / panel_height;
    let doc_ratio = doc_width / doc_height;
    let ratio = if panel_ratio < doc_ratio {
        panel_height / doc_height / panel_ratio
    } else {
        panel_width / doc_width / panel_ratio
    };

    // save the document as an image buffer
    let pixmap = typst_render::render(&document.pages[0], ratio);
    let pic_width = pixmap.width();
    let pic_height = pixmap.height();
    let pic: Vec<u8> = pixmap.take();

    for (pos, item) in document.pages[0].frame.items() {
        if let FrameItem::Group(group) = item {
            item_positions.push((Pos2::default(), Pos2::default()));

            let mut final_rect = Rect::from_pos(Pos2::new(
                ratio * pos.x.to_pt() as f32,
                ratio * pos.y.to_pt() as f32,
            ));
            final_rect.set_width(doc_width * ratio);
            final_rect.set_height((group.frame.height().to_pt() as f32) * ratio);
            ui.painter().add(Shape::rect_stroke(
                final_rect,
                5.,
                egui::Stroke::new(2., Color32::RED),
                egui::StrokeKind::Inside,
            ));
            println!(
                "Group info: {:?} - Group size: {:?}",
                pos,
                group.frame.size()
            );
        }

        // item_positions.push((x_current_group, y_current_group));
    }
    (pic_width, pic_height, pic)
}

impl eframe::App for App {
    fn clear_color(&self, _: &Visuals) -> [f32; 4] {
        [0.0, 0.0, 0.0, 0.0]
    }

    fn update(&mut self, ctx: &Context, _: &mut eframe::Frame) {
        CentralPanel::default()
            // .frame(egui::Frame::default().inner_margin(0.0).outer_margin(0.0))
            .show(ctx, |ui| {
                ui.input(|i| {
                    if i.key_pressed(egui::Key::Q) {
                        let ctx = ctx.clone();
                        std::thread::spawn(move || {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        });
                    }
                });

                // get information about the document
                let (page_width, page_height, pic) = document_info(ui);

                let final_img = ColorImage::from_rgba_unmultiplied(
                    [page_width as usize, page_height as usize],
                    pic.as_slice(),
                );

                // set the background image derived from the typst document
                self.texture.set(final_img, egui::TextureOptions::NEAREST);
                let size = self.texture.size_vec2();
                let sized_texture = egui::load::SizedTexture::new(&self.texture, size);
                ui.add(egui::Image::new(sized_texture));

                // for frame in item_positions.iter() {
                // println!("{}, {}", item_positions[0].0, item_positions[0].1);
                // ui.painter().add(Shape::rect_stroke(
                //     Rect::from_two_pos(item_positions[0].0, item_positions[0].1),
                //     5.,
                //     egui::Stroke::new(5., Color32::RED),
                //     egui::StrokeKind::Inside,
                // ));
                // }

                // debug helpers
                if cfg!(debug_assertions) {
                    debug_output(ui);
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

fn debug_output(ui: &mut Ui) {
    ui.put(
        Rect::from_min_max((0., 0.).into(), (100., 50.).into()),
        Label::new(
            ui.ctx()
                .pointer_latest_pos()
                .unwrap_or(Pos2 { x: -1., y: -1. })
                .to_string(),
        ),
    );
}
