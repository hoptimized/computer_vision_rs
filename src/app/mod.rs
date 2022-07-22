use image::{DynamicImage, GrayImage};
use rfd::FileHandle;
use std::future::Future;
use std::sync::mpsc;

mod image_frame;
mod operations;
mod panels;

use image_frame::ImageFrame;
use panels::TopPanel;

pub enum GuiAction {
    OpenFileDialog,
    ApplyGrayscale,
    ApplyInvert,
    AcceptOperation,
    DiscardOperation,
    ResetAll,

    LoadImage(Option<FileHandle>),
    ImageLoaded(DynamicImage),
}

pub trait View {
    fn show(&mut self, ctx: &egui::Context);
    fn ui(&mut self, ui: &mut egui::Ui);
}

/// We derive Deserialize/Serialize so we can persist app state on shutdown. (currently not used)
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct MyApp {
    #[serde(skip)]
    message_channel: (mpsc::Sender<GuiAction>, mpsc::Receiver<GuiAction>),

    #[serde(skip)]
    top_panel: TopPanel,

    #[serde(skip)]
    view_current: ImageFrame,
    #[serde(skip)]
    view_preview: ImageFrame,

    #[serde(skip)]
    model_current: Option<DynamicImage>,
    #[serde(skip)]
    model_preview: Option<DynamicImage>,
}

impl Default for MyApp {
    fn default() -> Self {
        let (tx, rx) = mpsc::channel();

        Self {
            message_channel: (tx.clone(), rx),
            top_panel: TopPanel::new(tx.clone()),
            view_current: ImageFrame::new("Current", true, tx.clone()),
            view_preview: ImageFrame::new("Preview", false, tx),
            model_current: None,
            model_preview: None,
        }
    }
}

impl MyApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Load previous app state (if any)
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }

    fn open_file_dialog(&mut self) {
        let task = rfd::AsyncFileDialog::new()
            .add_filter("Image files", &["png", "jpg", "jpeg"])
            .set_directory("/")
            .pick_file();

        let tx = self.message_channel.0.clone();

        execute(async move {
            let file = task.await;
            tx.send(GuiAction::LoadImage(file)).ok();
        });
    }

    fn load_new_image(&mut self, file: Option<FileHandle>) {
        let tx = self.message_channel.0.clone();
        execute(async move {
            if let Some(file) = file {
                let data = file.read().await;

                if let Ok(image) = image::load_from_memory(&*data) {
                    tx.send(GuiAction::ImageLoaded(image)).ok();
                }
            }
        });
    }

    fn preview_operation(&mut self, func: fn(image: &DynamicImage) -> Option<GrayImage>) {
        if let Some(model_current) = &self.model_current {
            if let Some(transformed) = func(model_current) {
                let transformed = DynamicImage::ImageLuma8(transformed);
                self.model_preview = Some(transformed);

                self.view_preview.set_image(self.model_preview.clone());
            }
        }
    }

    fn accept_operation(&mut self) {
        if self.model_preview.is_some() {
            self.model_current = self.model_preview.take();

            self.view_current.set_image(self.model_current.clone());
            self.view_preview.set_image(self.model_preview.clone());
        }
    }

    fn discard_operation(&mut self) {
        self.model_preview = None;

        self.view_preview.set_image(self.model_preview.clone());
    }

    fn reset(&mut self, new_image: Option<DynamicImage>) {
        self.model_current = new_image;
        self.model_preview = None;

        self.view_current.set_image(self.model_current.clone());
        self.view_preview.set_image(self.model_preview.clone());
    }
}

impl eframe::App for MyApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        while let Ok(message) = self.message_channel.1.try_recv() {
            match message {
                GuiAction::OpenFileDialog => self.open_file_dialog(),
                GuiAction::LoadImage(file) => self.load_new_image(file),
                GuiAction::ImageLoaded(new_image) => self.reset(Some(new_image)),
                GuiAction::ApplyGrayscale => self.preview_operation(operations::grayscale),
                GuiAction::ApplyInvert => self.preview_operation(operations::invert),
                GuiAction::AcceptOperation => self.accept_operation(),
                GuiAction::DiscardOperation => self.discard_operation(),
                GuiAction::ResetAll => self.reset(None),
            };
        }

        self.top_panel.set_has_current(self.model_current.is_some());
        self.top_panel.set_has_preview(self.model_preview.is_some());
        self.top_panel.show(ctx);

        egui::CentralPanel::default().show(ctx, |_ui| {
            self.view_current.show(ctx);
            self.view_preview.show(ctx);
        });
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn execute<F: Future<Output = ()> + Send + 'static>(f: F) {
    std::thread::spawn(move || futures::executor::block_on(f));
}
#[cfg(target_arch = "wasm32")]
fn execute<F: Future<Output = ()> + 'static>(f: F) {
    wasm_bindgen_futures::spawn_local(f);
}
