use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use egui::Vec2;
use itertools::Itertools;
use typst::{
    Library, World,
    diag::{FileError, FileResult},
    foundations::{Bytes, Datetime},
    layout::{FrameItem, Page, PagedDocument},
    syntax::{FileId, Source},
    text::{Font, FontBook},
    utils::LazyHash,
};
use typst_kit::fonts::{FontSearcher, FontSlot};

pub struct TypstWorld {
    source: Source,
    library: LazyHash<Library>,
    book: LazyHash<FontBook>,
    #[cfg(not(target_arch = "wasm32"))]
    root: std::path::PathBuf,
    files: Arc<Mutex<HashMap<FileId, FileEntry>>>,
    fonts: Vec<FontSlot>,
    #[cfg(target_arch = "wasm32")]
    content_dir: include_dir::Dir<'static>,
}

pub struct DocumentPage {
    pub page: Page,
    pub image: Image,
    pub ratio_page_to_panel: f32,
}

pub struct Image {
    pub data: Vec<u8>,
    pub width: u32,
    pub height: u32,
}

pub struct DataBlock {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub label: String,
}

impl DocumentPage {
    #[cfg(not(target_arch = "wasm32"))]
    pub fn new(input: &str, panel_size: Vec2) -> Result<Self, String> {
        let world = TypstWorld::new(input.to_string());
        DocumentPage::new_followup(world, panel_size)
    }

    #[cfg(target_arch = "wasm32")]
    pub fn new(
        input: &str,
        panel_size: Vec2,
        content_dir: include_dir::Dir<'static>,
    ) -> Result<Self, String> {
        let world = TypstWorld::new(input.to_string(), content_dir);
        DocumentPage::new_followup(world, panel_size)
    }

    pub fn new_followup(mut world: TypstWorld, panel_size: Vec2) -> Result<Self, String> {
        let world_source = &mut world.source;

        // flip to horizontal if width is bigger than height
        if panel_size.x > panel_size.y {
            if let Some((line_idx, line_text)) = world_source
                .text()
                .lines()
                .find_position(|t| t.contains("flipped"))
            {
                let replacement = line_text.replace("false", "true");
                world_source.edit(
                    world_source.line_to_range(line_idx).unwrap(),
                    replacement.as_str(),
                );
            }

            // change the style_portrait.typ file to style_landscape.typ if they exist
            if let Some((line_idx, line_text)) = world_source
                .text()
                .lines()
                .find_position(|t| t.contains("style_portrait.typ"))
            {
                let replacement = line_text.replace("portrait", "landscape");
                world_source.edit(world_source.line_to_range(line_idx).unwrap(), &replacement);
            }
        }

        // compile the document
        let document: PagedDocument = typst::compile(&world).output.expect("Typst compile error.");

        if document.pages.is_empty() {
            return Err("No pages found".to_string());
        }

        // for now only take the first page
        let page = document.pages[0].clone();

        let width = page.frame.size().x.to_pt() as f32;
        let height = page.frame.size().y.to_pt() as f32;
        let page_ratio = width / height;

        let (panel_width, panel_height) = (panel_size.x, panel_size.y);
        let panel_ratio = panel_width / panel_height;

        let ratio_page_to_panel = if panel_ratio > page_ratio {
            panel_height / height
        } else {
            panel_width / width
        };

        // convert the page to image data using the ratio between the page and the canvas geometry
        let pixmap = typst_render::render(&page, ratio_page_to_panel);
        let pic_width = pixmap.width();
        let pic_height = pixmap.height();
        let image = Image {
            data: pixmap.take(),
            width: pic_width,
            height: pic_height,
        };

        Ok(Self {
            page,
            image,
            ratio_page_to_panel,
        })
    }

    pub fn as_vec(&self) -> &Vec<u8> {
        &self.image.data
    }

    fn filter_for_relevant_blocks(
        &self,
        frame: &typst::layout::Frame,
        mut blocks: Vec<DataBlock>,
        offset: typst::layout::Point,
    ) -> Vec<DataBlock> {
        let mut grid_found = false;
        let mut label_takeover = String::new();
        let outset = 8.;

        for (pos, item) in frame.items() {
            match item {
                FrameItem::Group(group_item) => {
                    if grid_found {
                        grid_found = false;
                        let pos = typst::layout::Point::new(pos.x + offset.x, pos.y + offset.y);

                        let block = DataBlock {
                            x: self.ratio_page_to_panel * pos.x.to_pt() as f32 - outset,
                            y: self.ratio_page_to_panel * pos.y.to_pt() as f32 - outset,
                            width: group_item.frame.width().to_pt() as f32
                                * self.ratio_page_to_panel
                                + (2. * outset),
                            height: group_item.frame.height().to_pt() as f32
                                * self.ratio_page_to_panel
                                + (2. * outset),
                            label: label_takeover.clone(),
                        };

                        blocks.push(block);
                        label_takeover.clear();
                    } else {
                        let offset = typst::layout::Point {
                            x: offset.x + pos.x,
                            y: offset.y + pos.y,
                        };
                        blocks = self.filter_for_relevant_blocks(&group_item.frame, blocks, offset);
                    }
                }
                FrameItem::Tag(typst::introspection::Tag::Start(content)) => {
                    if content.elem().name() == "grid" {
                        grid_found = true;
                        let label = content.label().unwrap().resolve();
                        label_takeover = String::from(label.as_str());
                    }
                }
                _ => (),
            }
        }

        blocks
    }

    pub fn get_data_blocks(&self) -> Vec<DataBlock> {
        let mut blocks = Vec::new();
        let offset = typst::layout::Point::zero();
        blocks = self.filter_for_relevant_blocks(&self.page.frame, blocks, offset);

        blocks
    }
}

#[derive(Clone, Debug)]
struct FileEntry {
    bytes: Bytes,
    source: Option<Source>,
}

impl FileEntry {
    fn new(bytes: Vec<u8>, source: Option<Source>) -> Self {
        Self {
            bytes: Bytes::new(bytes),
            source,
        }
    }

    fn source(&mut self, id: FileId) -> FileResult<Source> {
        let source = if let Some(source) = &self.source {
            source
        } else {
            let contents = std::str::from_utf8(&self.bytes).map_err(|_| FileError::InvalidUtf8)?;
            let contents = contents.trim_start_matches('\u{feff}');
            let source = Source::new(id, contents.into());
            self.source.insert(source)
        };
        Ok(source.clone())
    }
}

impl TypstWorld {
    #[cfg(not(target_arch = "wasm32"))]
    fn new(source: String) -> Self {
        let fonts = FontSearcher::new().include_system_fonts(true).search();
        Self {
            source: Source::detached(source),
            library: LazyHash::new(Library::default()),
            book: LazyHash::new(fonts.book),
            root: std::path::PathBuf::from("./assets/"),
            files: Arc::new(Mutex::new(HashMap::new())),
            fonts: fonts.fonts,
        }
    }

    #[cfg(target_arch = "wasm32")]
    fn new(source: String, content_dir: include_dir::Dir<'static>) -> Self {
        let fonts = FontSearcher::new().include_system_fonts(true).search();
        Self {
            source: Source::detached(source),
            library: LazyHash::new(Library::default()),
            book: LazyHash::new(fonts.book),
            files: Arc::new(Mutex::new(HashMap::new())),
            fonts: fonts.fonts,
            content_dir,
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn file(&self, id: FileId) -> FileResult<FileEntry> {
        let mut files = self.files.lock().map_err(|_| FileError::AccessDenied)?;
        if let Some(entry) = files.get(&id) {
            return Ok(entry.clone());
        }

        let path = id
            .vpath()
            .resolve(&self.root)
            .ok_or(FileError::AccessDenied)?;

        let content = std::fs::read(&path).map_err(|error| FileError::from_io(error, &path))?;

        Ok(files
            .entry(id)
            .or_insert(FileEntry::new(content, None))
            .clone())
    }

    #[cfg(target_arch = "wasm32")]
    fn file(&self, id: FileId) -> FileResult<FileEntry> {
        let mut files = self.files.lock().map_err(|_| FileError::AccessDenied)?;
        if let Some(entry) = files.get(&id) {
            return Ok(entry.clone());
        }

        let path = id
            .vpath()
            .as_rootless_path()
            .to_str()
            .ok_or(FileError::AccessDenied)?;

        let content = self.content_dir.get_file(path).unwrap();
        Ok(files
            .entry(id)
            .or_insert(FileEntry::new(content.contents().to_vec(), None))
            .clone())
    }
}

impl World for TypstWorld {
    #[doc = " The standard library."]
    #[doc = ""]
    #[doc = " Can be created through `Library::build()`."]
    fn library(&self) -> &LazyHash<Library> {
        &self.library
    }

    #[doc = " Metadata about all known fonts."]
    fn book(&self) -> &LazyHash<FontBook> {
        &self.book
    }

    #[doc = " Get the file id of the main source file."]
    fn main(&self) -> FileId {
        self.source.id()
    }

    #[doc = " Try to access the specified source file."]
    fn source(&self, id: FileId) -> FileResult<Source> {
        if id == self.source.id() {
            Ok(self.source.clone())
        } else {
            self.file(id)?.source(id)
        }
    }

    #[doc = " Try to access the specified file."]
    fn file(&self, id: FileId) -> FileResult<Bytes> {
        self.file(id).map(|file| file.bytes.clone())
    }

    #[doc = " Try to access the font with the given index in the font book."]
    fn font(&self, index: usize) -> Option<Font> {
        self.fonts[index].get()
    }

    #[doc = " Get the current date."]
    #[doc = ""]
    #[doc = " If no offset is specified, the local date should be chosen. Otherwise,"]
    #[doc = " the UTC date should be chosen with the corresponding offset in hours."]
    #[doc = ""]
    #[doc = " If this function returns `None`, Typst\'s `datetime` function will"]
    #[doc = " return an error."]
    fn today(&self, _offset: Option<i64>) -> Option<Datetime> {
        todo!()
    }
}
