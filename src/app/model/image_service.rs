use crate::app::model::image::Image;
use crate::app::model::operations;
use image::{DynamicImage, GrayImage};
use rfd::FileHandle;
use std::sync::{mpsc, Arc};

enum Message {
    ImageLoaded(DynamicImage),
}

pub struct ImageService {
    message_channel: (mpsc::Sender<Message>, mpsc::Receiver<Message>), // TODO: turn into promise
    current_image: Arc<Image>,
    preview_image: Arc<Image>,
}

impl ImageService {
    pub fn new() -> Self {
        Self {
            message_channel: mpsc::channel(),
            current_image: Arc::new(Image::new()),
            preview_image: Arc::new(Image::new()),
        }
    }

    pub fn update(&self) {
        while let Ok(message) = self.message_channel.1.try_recv() {
            match message {
                Message::ImageLoaded(new_image) => self.reset(Some(new_image)),
            }
        }
        while self.message_channel.1.try_recv().is_ok() {}
    }

    pub fn get_current_image(&self) -> Arc<Image> {
        Arc::clone(&self.current_image)
    }

    pub fn get_preview_image(&self) -> Arc<Image> {
        Arc::clone(&self.preview_image)
    }

    pub fn load_new_image(&self, file: Option<FileHandle>) {
        let tx = self.message_channel.0.clone();
        crate::app::execute(async move {
            if let Some(file) = file {
                let data = file.read().await;
                if let Ok(image) = image::load_from_memory(&*data) {
                    tx.send(Message::ImageLoaded(image)).ok();
                }
            }
        });
    }

    pub fn apply_grayscale(&self) {
        self.preview_operation(operations::grayscale);
    }

    pub fn apply_invert(&self) {
        self.preview_operation(operations::invert);
    }

    pub fn accept_operation(&self) {
        let preview_image = self.preview_image.get();
        if preview_image.is_some() {
            self.preview_image.set(None);
            self.current_image.set_arc(preview_image);
        }
    }

    pub fn discard_operation(&self) {
        self.preview_image.set(None);
    }

    pub fn reset(&self, new_image: Option<DynamicImage>) {
        self.current_image.set(new_image);
        self.preview_image.set(None);
    }

    fn preview_operation(&self, func: fn(image: &DynamicImage) -> Option<GrayImage>) {
        let current_image = &*self.current_image.get();
        if let Some(current_image) = current_image {
            if let Some(transformed) = func(current_image) {
                let transformed = DynamicImage::ImageLuma8(transformed);
                self.preview_image.set(Some(transformed));
            }
        }
    }
}
