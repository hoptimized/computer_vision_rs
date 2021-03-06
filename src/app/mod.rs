use rfd::FileHandle;
use std::future::Future;
use std::sync::{mpsc, Arc};

mod model;
mod view;
mod viewmodel;

use model::ImageService;

pub enum GuiAction {
    OpenFileDialog,
    LoadImage(Option<FileHandle>),
}

/// We derive Deserialize/Serialize so we can persist app state on shutdown. (currently not used)
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct MyApp {
    #[serde(skip)]
    message_channel: (mpsc::Sender<GuiAction>, mpsc::Receiver<GuiAction>),

    #[serde(skip)]
    views: Vec<Box<dyn view::View>>,

    #[serde(skip)]
    image_service: Arc<ImageService>,
}

impl Default for MyApp {
    fn default() -> Self {
        let image_service = Arc::new(ImageService::new());

        let model_current = image_service.get_current_image();
        let model_preview = image_service.get_preview_image();

        let (tx, rx) = mpsc::channel(); // TODO: remove this

        let views: Vec<Box<dyn view::View>> = vec![
            Box::new(view::TopPanel::new(viewmodel::TopPanel::new(
                tx.clone(),
                Arc::clone(&image_service),
                Arc::clone(&model_current),
                Arc::clone(&model_preview),
            ))),
            Box::new(view::CentralPanel::new(vec![
                Box::new(view::ImageFrame::new(viewmodel::ImageFrame::new(
                    "Current",
                    true,
                    tx.clone(),
                    model_current.clone(),
                ))),
                Box::new(view::ImageFrame::new(viewmodel::ImageFrame::new(
                    "Preview",
                    false,
                    tx.clone(),
                    model_preview.clone(),
                ))),
            ])),
        ];

        Self {
            message_channel: (tx, rx),
            views,
            image_service,
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
                GuiAction::LoadImage(file) => self.image_service.load_new_image(file),
            };
        }

        self.image_service.update();

        for view in self.views.iter_mut() {
            view.show(ctx);
        }
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
