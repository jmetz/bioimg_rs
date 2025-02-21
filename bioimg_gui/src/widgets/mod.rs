use std::fmt::Display;

use self::{error_display::show_if_error, util::group_frame};
use crate::result::{GuiError, Result};

pub mod author_widget;
pub mod axis_size_widget;
pub mod cite_widget;
pub mod code_editor_widget;
pub mod cover_image_widget;
pub mod error_display;
pub mod example_tensor_widget;
pub mod file_widget;
pub mod functional;
pub mod icon_widget;
pub mod input_tensor_widget;
pub mod maintainer_widget;
pub mod tensor_axis_widget;
pub mod url_widget;
pub mod util;
pub mod enum_widget;

pub trait StatefulWidget {
    type Value<'p>
    where
        Self: 'p;
    fn draw_and_parse(&mut self, ui: &mut egui::Ui, id: egui::Id);
    fn state<'p>(&'p self) -> Self::Value<'p>;
}

pub struct StagingNum<N, T> {
    pub raw: N,
    pub parsed: Result<T>,
}

impl<N, T> Default for StagingNum<N, T>
where
    N: Default,
    T: TryFrom<N>,
    T::Error: Display,
{
    fn default() -> Self {
        Self {
            raw: N::default(),
            parsed: T::try_from(N::default()).map_err(|err| GuiError::new(err.to_string())),
        }
    }
}

impl<N, T> StatefulWidget for StagingNum<N, T>
where
    N: egui::emath::Numeric,
    T: TryFrom<N> + Clone,
    T::Error: Display + Clone,
{
    type Value<'p> = Result<T> where T: 'p;

    fn draw_and_parse(&mut self, ui: &mut egui::Ui, _id: egui::Id) {
        ui.add(egui::widgets::DragValue::new(&mut self.raw));
        self.parsed = T::try_from(self.raw.clone()).map_err(|err| GuiError::new(err.to_string()));
        show_if_error(ui, &self.parsed);
    }

    fn state<'p>(&'p self) -> Self::Value<'p> {
        self.parsed.clone()
    }
}

#[derive(Clone, Debug)]
pub enum InputLines {
    SingleLine,
    Multiline,
}

#[derive(Debug)]
pub struct StagingString<T> {
    raw: String,
    parsed: Result<T>,
    input_lines: InputLines,
}

impl<T> Default for StagingString<T>
where
    T: TryFrom<String>,
    T::Error: Display,
{
    fn default() -> Self {
        let raw = String::default();
        Self {
            raw: raw.clone(),
            parsed: T::try_from(raw).map_err(|err| GuiError::new(err.to_string())),
            input_lines: InputLines::SingleLine,
        }
    }
}

impl<T> StagingString<T>
where
    T: TryFrom<String>,
    T::Error: Display,
{
    pub fn new(input_lines: InputLines) -> Self {
        let raw = String::default();
        Self {
            raw: raw.clone(),
            parsed: T::try_from(raw).map_err(|err| GuiError::new(err.to_string())),
            input_lines,
        }
    }
}

impl<T> StatefulWidget for StagingString<T>
where
    T: TryFrom<String> + Clone,
    T::Error: Display,
{
    type Value<'p> = Result<T> where T: 'p;

    fn draw_and_parse<'p>(&'p mut self, ui: &mut egui::Ui, _id: egui::Id) {
        ui.horizontal(|ui| {
            match self.input_lines {
                InputLines::SingleLine => {
                    ui.add(
                        //FIXME: any way we can not hardcode this? at least use font size?
                        egui::TextEdit::singleline(&mut self.raw).min_size(egui::Vec2 { x: 200.0, y: 10.0 }),
                    );
                }
                InputLines::Multiline => {
                    ui.text_edit_multiline(&mut self.raw);
                }
            }
            self.parsed = T::try_from(self.raw.clone()).map_err(|err| GuiError::new(err.to_string()));
            show_if_error(ui, &self.parsed);
        });
    }

    fn state<'p>(&'p self) -> Self::Value<'p> {
        self.parsed.clone()
    }
}

#[derive(Clone, Debug, Default)]
pub struct StagingOpt<Stg: StatefulWidget>(Option<Stg>);

impl<Stg> StatefulWidget for StagingOpt<Stg>
where
    Stg: Default + StatefulWidget,
{
    type Value<'p> = Option<Stg::Value<'p>>
    where
        Stg::Value<'p>: 'p,
        Stg: 'p;

    fn draw_and_parse<'p>(&'p mut self, ui: &mut egui::Ui, id: egui::Id) {
        ui.horizontal(|ui| {
            if self.0.is_none() {
                ui.label("None");
                if ui.button("Add").clicked() {
                    self.0 = Some(Stg::default())
                }
            } else {
                let x_clicked = ui.button("🗙").clicked();
                group_frame(ui, |ui| {
                    self.0.as_mut().unwrap().draw_and_parse(ui, id);
                });
                if x_clicked {
                    self.0.take();
                }
            }
        });
    }

    fn state<'p>(&'p self) -> Self::Value<'p> {
        self.0.as_ref().map(|inner_widget| inner_widget.state())
    }
}

pub struct StagingVec<Stg>
where
    Stg: StatefulWidget,
{
    pub item_name: String,
    pub staging: Vec<Stg>,
}

impl<Stg: StatefulWidget + Default> StagingVec<Stg> {
    pub fn new(item_name: impl Into<String>) -> Self {
        Self {
            staging: vec![Stg::default()],
            item_name: item_name.into(),
        }
    }
}

impl<Stg: StatefulWidget> StatefulWidget for StagingVec<Stg>
where
    Stg: Default,
{
    type Value<'p> = Vec<Stg::Value<'p>>
    where
        Stg: 'p,
        Stg::Value<'p>: 'p;

    fn draw_and_parse<'p>(&'p mut self, ui: &mut egui::Ui, id: egui::Id) {
        let item_name = &self.item_name;
        ui.vertical(|ui| {
            self.staging.iter_mut().enumerate().for_each(|(idx, staging_item)| {
                ui.label(format!("{item_name} #{}", idx + 1));
                group_frame(ui, |ui| {
                    staging_item.draw_and_parse(ui, id.with(idx));
                });
            });
            ui.horizontal(|ui| {
                if ui.button(format!("+ Add {item_name}")).clicked() {
                    self.staging.resize_with(self.staging.len() + 1, Stg::default);
                }
                if ui.button(format!("- Remove {item_name}")).clicked() && self.staging.len() > 1 {
                    self.staging.resize_with(self.staging.len() - 1, Stg::default);
                }
            });
        });
    }

    fn state<'p>(&'p self) -> Self::Value<'p> {
        self.staging.iter().map(|item_widget| item_widget.state()).collect()
    }
}
