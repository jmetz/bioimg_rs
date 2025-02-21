use bioimg_spec::rdf;

use super::{util::DynamicImageExt, StagingString, StatefulWidget};

use crate::result::Result;
use std::path::PathBuf;

use bioimg_spec::runtime as rt;
use egui::{load::SizedTexture, ImageSource};

use super::{
    error_display::show_error,
    file_widget::{FileWidget, ParsedFile},
};

pub struct GuiIconImage {
    path: PathBuf,
    contents: rt::Icon,
    context: egui::Context,
    texture_handle: egui::TextureHandle,
}

impl Drop for GuiIconImage {
    fn drop(&mut self) {
        self.context.forget_image(&self.path.to_string_lossy());
    }
}

impl ParsedFile for Result<GuiIconImage> {
    fn parse(path: PathBuf, ctx: egui::Context) -> Self {
        let img = image::io::Reader::open(&path)?.decode()?;
        let icon = rt::Icon::try_from(img.clone())?;
        let texture_handle = img.to_egui_texture_handle(path.to_string_lossy(), &ctx);
        Ok(GuiIconImage {
            path: path.clone(),
            contents: icon,
            context: ctx,
            texture_handle: texture_handle.clone(),
        })
    }

    fn render(&self, ui: &mut egui::Ui, id: egui::Id) {
        match self {
            Ok(loaded_cover_image) => {
                let image_source = ImageSource::Texture(SizedTexture {
                    id: loaded_cover_image.texture_handle.id(),
                    size: egui::Vec2 { x: 20.0, y: 20.0 },
                });
                let ui_img = egui::Image::new(image_source);
                ui.add(ui_img);
            }
            Err(err) => show_error(ui, err.to_string()),
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
enum InputMode {
    Emoji,
    File,
}

impl Default for InputMode {
    fn default() -> Self {
        Self::Emoji
    }
}

#[derive(Default)]
pub struct StagingIcon {
    emoji_icon_widget: StagingString<rdf::Icon>,
    image_icon_widget: FileWidget<Result<GuiIconImage>>,
    input_mode: InputMode,
}

impl StatefulWidget for StagingIcon {
    type Value<'p> = Result<rt::Icon>;

    fn draw_and_parse(&mut self, ui: &mut egui::Ui, id: egui::Id) {
        ui.vertical(|ui| {
            ui.horizontal(|ui| {
                ui.radio_value(&mut self.input_mode, InputMode::Emoji, "Emoji Icon");
                ui.radio_value(&mut self.input_mode, InputMode::File, "Image File Icon");
            });
            if self.input_mode == InputMode::Emoji {
                self.emoji_icon_widget.draw_and_parse(ui, id.with("Emoji Icon"));
            }
            if self.input_mode == InputMode::File {
                self.image_icon_widget.draw_and_parse(ui, id.with("Image File Icon"));
            };
        });
    }

    fn state<'p>(&'p self) -> Self::Value<'p> {
        unimplemented!("Create a rt::Icon")
    }
}
