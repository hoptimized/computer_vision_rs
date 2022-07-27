use crate::app::model;
use crate::app::model::ImageService;
use image::DynamicImage;
use rfd::FileHandle;
use std::sync::Arc;
use tokio::sync::broadcast;

#[derive(Clone)]
#[allow(dead_code)]
pub enum PropertyChangedNotification {
    AcceptInput,
    Image,
    Open,
    Title,
}

pub struct ImageFrame {
    view_channel: (
        broadcast::Sender<PropertyChangedNotification>,
        broadcast::Receiver<PropertyChangedNotification>,
    ),

    // properties
    accept_input: bool,
    image: Arc<Option<DynamicImage>>,
    open: bool,
    title: String,

    // dependencies
    image_service: Arc<ImageService>,
    model: Arc<model::Image>,
    model_rx: broadcast::Receiver<()>,
}

impl ImageFrame {
    pub fn new(
        title: &str,
        accept_input: bool,
        image_service: Arc<ImageService>,
        model: Arc<model::Image>,
    ) -> Self {
        let model_rx = model.get_property_changed_rx();

        Self {
            view_channel: broadcast::channel(32),
            title: title.to_string(),
            image: Arc::new(None),
            accept_input,
            open: true,
            image_service,
            model,
            model_rx,
        }
    }

    pub fn process_messages(&mut self) {
        if self.model_rx.try_recv().is_ok() {
            self.set_image(self.model.get());
        }
    }

    pub fn get_receiver(&self) -> broadcast::Receiver<PropertyChangedNotification> {
        self.view_channel.0.subscribe()
    }

    pub fn open_file(&self, file: Option<FileHandle>) {
        self.image_service.load_new_image(file);
    }

    pub fn get_accept_input(&self) -> bool {
        self.accept_input
    }

    pub fn get_image(&self) -> Arc<Option<DynamicImage>> {
        self.model.get()
    }

    pub fn get_open(&self) -> bool {
        self.open
    }

    pub fn get_title(&self) -> &String {
        &self.title
    }

    #[allow(dead_code)]
    pub fn set_accept_input(&mut self, accept_input: bool) {
        self.accept_input = accept_input;
        self.view_channel
            .0
            .send(PropertyChangedNotification::AcceptInput)
            .ok();
    }

    #[allow(dead_code)]
    pub fn set_image(&mut self, image: Arc<Option<DynamicImage>>) {
        self.image = image;
        self.view_channel
            .0
            .send(PropertyChangedNotification::Image)
            .ok();
    }

    #[allow(dead_code)]
    pub fn set_open(&mut self, open: bool) {
        self.open = open;
        self.view_channel
            .0
            .send(PropertyChangedNotification::Open)
            .ok();
    }

    #[allow(dead_code)]
    pub fn set_title(&mut self, title: String) {
        self.title = title;
        self.view_channel
            .0
            .send(PropertyChangedNotification::Title)
            .ok();
    }
}
