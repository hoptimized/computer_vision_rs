use egui::{ColorImage, Context, Ui};
use egui_extras::RetainedImage;
use image::DynamicImage;
use std::sync::mpsc;

use super::{GuiAction, View};

pub struct ImageFrame {
    title: String,
    image: Option<RetainedImage>,

    accept_input: bool,
    open: bool,

    tx: mpsc::Sender<GuiAction>,
}

impl ImageFrame {
    pub fn new(title: &str, accept_input: bool, tx: mpsc::Sender<GuiAction>) -> Self {
        Self {
            title: title.to_string(),
            image: None,
            accept_input,
            open: true,
            tx,
        }
    }

    pub fn set_image(&mut self, image: Option<DynamicImage>) {
        match image {
            Some(image) => {
                let size = [image.width() as _, image.height() as _];
                let image_buffer = image.to_rgba8();
                let pixels = image_buffer.as_flat_samples();
                let color_image = ColorImage::from_rgba_unmultiplied(size, pixels.as_slice());
                self.image
                    .replace(RetainedImage::from_color_image("foo", color_image));
            }
            None => self.image = None,
        }
    }

    fn open_file_dialog(&self) {
        self.tx.send(GuiAction::OpenFileDialog).ok();
    }
}

impl View for ImageFrame {
    fn show(&mut self, ctx: &Context) {
        let mut open = self.open;
        let title = self.title.clone();

        egui::Window::new(title)
            .open(&mut open)
            //.closable(false)
            .collapsible(false)
            .resizable(false)
            .default_width(300.0)
            .default_height(300.0)
            .show(ctx, |ui| self.ui(ui));

        self.open = open;
    }

    fn ui(&mut self, ui: &mut Ui) {
        let Self { image, .. } = self;

        match image {
            Some(image) => {
                let size = image.size();
                let scale = 1f32 / (size[0].max(size[1]) as f32 / 300f32);
                image.show_scaled(ui, scale);
            }
            _ => {
                ui.label("nothing to show");
                if self.accept_input && ui.add(egui::widgets::Button::new("Open Image")).clicked() {
                    self.open_file_dialog();
                }
            }
        }
    }
}
