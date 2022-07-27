use crate::app::model;
use crate::app::model::ImageService;
use rfd::FileHandle;
use std::sync::Arc;
use tokio::sync::broadcast;

#[derive(Clone)]
pub enum PropertyChangedNotification {
    HasCurrent,
    HasPreview,
}

pub struct TopPanel {
    view_channel: (
        broadcast::Sender<PropertyChangedNotification>,
        broadcast::Receiver<PropertyChangedNotification>,
    ),

    // properties
    has_current: bool,
    has_preview: bool,

    // dependencies
    image_service: Arc<ImageService>,
    current_image: Arc<model::Image>,
    preview_image: Arc<model::Image>,
    current_image_rx: broadcast::Receiver<()>,
    preview_image_rx: broadcast::Receiver<()>,
}

impl TopPanel {
    pub fn new(
        image_service: Arc<ImageService>,
        current_image: Arc<model::Image>,
        preview_image: Arc<model::Image>,
    ) -> Self {
        let current_image_rx = current_image.get_property_changed_rx();
        let preview_image_rx = preview_image.get_property_changed_rx();

        Self {
            view_channel: broadcast::channel(32),
            has_current: false,
            has_preview: false,
            image_service,
            current_image,
            preview_image,
            current_image_rx,
            preview_image_rx,
        }
    }

    pub fn open_file(&mut self, file: Option<FileHandle>) {
        self.image_service.load_new_image(file);
    }

    pub fn process_messages(&mut self) {
        if self.current_image_rx.try_recv().is_ok() {
            self.set_has_current(self.current_image.get().is_some());
        }

        if self.preview_image_rx.try_recv().is_ok() {
            self.set_has_preview(self.preview_image.get().is_some());
        }
    }

    pub fn get_receiver(&self) -> broadcast::Receiver<PropertyChangedNotification> {
        self.view_channel.0.subscribe()
    }

    pub fn apply_grayscale(&mut self) {
        self.image_service.apply_grayscale();
    }

    pub fn apply_invert(&mut self) {
        self.image_service.apply_invert();
    }

    pub fn accept_operation(&mut self) {
        self.image_service.accept_operation();
    }

    pub fn discard_operation(&mut self) {
        self.image_service.discard_operation();
    }

    pub fn reset_images(&mut self) {
        self.image_service.reset(None);
    }

    pub fn get_has_current(&self) -> bool {
        self.has_current
    }

    pub fn get_has_preview(&self) -> bool {
        self.has_preview
    }

    fn set_has_current(&mut self, has_current: bool) {
        self.has_current = has_current;
        self.view_channel
            .0
            .send(PropertyChangedNotification::HasCurrent)
            .ok();
    }

    fn set_has_preview(&mut self, has_preview: bool) {
        self.has_preview = has_preview;
        self.view_channel
            .0
            .send(PropertyChangedNotification::HasPreview)
            .ok();
    }
}
