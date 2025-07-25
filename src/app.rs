use crate::{document::DocumentPage, overlay::Overlay};
use egui::{CentralPanel, Color32, ColorImage, Context, Pos2, Rect, Ui, Vec2, Visuals};

pub struct App {
    areas: Vec<Overlay>,
    texture: egui::TextureHandle,
    canvas_size: Vec2,
    document: DocumentPage,
    recompile_needed: bool,
}

impl App {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        cc.egui_ctx.set_visuals(Visuals {
            dark_mode: false,
            panel_fill: Color32::WHITE,
            ..Default::default()
        });

        Self {
            areas: Vec::new(),
            texture: cc.egui_ctx.load_texture(
                "background",
                egui::ColorImage::example(),
                egui::TextureOptions::NEAREST,
            ),
            canvas_size: Vec2::default(),
            document: get_document(cc.egui_ctx.screen_rect().size())
                .expect("Error with the typst document"),
            recompile_needed: true,
        }
    }

    fn recompile(&mut self) {
        // get the document
        self.document = get_document(self.canvas_size).expect("Error with typst document");

        // clear the stored data blocks
        self.areas.clear();

        // analyze the document
        let blocks = self.document.get_data_blocks();
        for block in &blocks {
            let mut final_rect = Rect::from_pos(Pos2::new(block.x, block.y));
            final_rect.set_width(block.width);
            final_rect.set_height(block.height);

            let new_area = Overlay::new(final_rect, block.label.clone(), self.canvas_size);
            self.areas.push(new_area);
        }

        // reset the flag
        self.recompile_needed = false;
    }
}

fn get_document(available_size: Vec2) -> Result<DocumentPage, String> {
    #[cfg(not(target_arch = "wasm32"))]
    {
        let content = std::include_str!("../assets/cv.typ");
        let document = DocumentPage::new(content, available_size)?;
        Ok(document)
    }

    #[cfg(target_arch = "wasm32")]
    {
        let content_dir = include_dir::include_dir!("./assets/cv/");
        let content_main_file = content_dir.get_file("main.typ").unwrap();
        let content = content_main_file.contents_utf8().unwrap();
        let document = DocumentPage::new(content, available_size, content_dir)?;
        Ok(document)
    }
}

fn render_background(
    ui: &mut Ui,
    document: &DocumentPage,
    texture_handle: &mut egui::TextureHandle,
) {
    let final_img = ColorImage::from_rgba_unmultiplied(
        [
            document.image.width as usize,
            document.image.height as usize,
        ],
        document.as_vec(),
    );

    // set the background image derived from the typst document
    texture_handle.set(final_img, egui::TextureOptions::NEAREST);
    let size = texture_handle.size_vec2();
    let sized_texture = egui::load::SizedTexture::new(texture_handle, size);
    ui.add(egui::Image::new(sized_texture));
}

impl eframe::App for App {
    fn clear_color(&self, _: &Visuals) -> [f32; 4] {
        [0.0, 0.0, 0.0, 0.0]
    }

    fn update(&mut self, ctx: &Context, _: &mut eframe::Frame) {
        // store the canvas size and retrigger the document compilation and area detection if it changed
        let size = ctx.screen_rect().size();
        if size != self.canvas_size {
            self.recompile_needed = true;
            self.canvas_size = size;
        }

        CentralPanel::default()
            .frame(egui::Frame::default().inner_margin(0.0).outer_margin(0.0))
            .show(ctx, |ui| {
                #[cfg(not(target_arch = "wasm32"))]
                ui.input(|i| {
                    if i.key_pressed(egui::Key::Q) {
                        let ctx = ctx.clone();
                        std::thread::spawn(move || {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        });
                    }
                });

                // only recompile and analyze the document on start and when the screen area changed
                if self.recompile_needed {
                    self.recompile();
                }

                // draw the document as a texture in the background
                render_background(ui, &self.document, &mut self.texture);

                // check for hovering areas and start the relevant animation
                for area in &mut self.areas {
                    ui.add(&mut *area);
                }
            });
    }
}
