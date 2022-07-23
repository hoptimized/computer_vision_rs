use super::View;
use crate::app::viewmodel;
use crate::app::viewmodel::image_frame::PropertyChangedNotification;
use egui::{ColorImage, Context, Ui};
use egui_extras::RetainedImage;
use image::DynamicImage;
use tokio::sync::broadcast;

pub struct ImageFrame {
    // properties
    accept_input: bool,
    image: Option<RetainedImage>,
    open: bool,
    title: String,

    // dependencies
    viewmodel: viewmodel::ImageFrame,
    vm_rx: broadcast::Receiver<PropertyChangedNotification>,
}

impl ImageFrame {
    pub fn new(viewmodel: viewmodel::ImageFrame) -> Self {
        let vm_rx = viewmodel.get_receiver();

        let mut result = Self {
            accept_input: viewmodel.get_accept_input(),
            image: None,
            open: viewmodel.get_open(),
            title: viewmodel.get_title().clone(),
            vm_rx,
            viewmodel,
        };

        result.set_image(&result.viewmodel.get_image());

        result
    }

    pub fn set_image(&mut self, image: &Option<DynamicImage>) {
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
                    self.viewmodel.open_file_dialog();
                }
            }
        }
    }
}

impl View for ImageFrame {
    fn show(&mut self, ctx: &Context) {
        self.viewmodel.process_messages();

        let mut open = self.open;
        let title = self.title.clone();

        if let Ok(notification) = self.vm_rx.try_recv() {
            match notification {
                PropertyChangedNotification::AcceptInput => {
                    self.accept_input = self.viewmodel.get_accept_input()
                }
                PropertyChangedNotification::Image => self.set_image(&*self.viewmodel.get_image()),
                PropertyChangedNotification::Open => self.open = self.viewmodel.get_open(),
                PropertyChangedNotification::Title => {
                    self.title = self.viewmodel.get_title().clone()
                }
            }
        }

        egui::Window::new(title)
            .open(&mut open)
            //.closable(false)
            .collapsible(false)
            .resizable(false)
            .default_width(300.0)
            .default_height(300.0)
            .show(ctx, |ui| self.ui(ui));

        self.viewmodel.set_open(open);
    }
}
