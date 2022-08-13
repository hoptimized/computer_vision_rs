use std::future::Future;
use std::sync::Arc;

mod modal;
mod model;
mod view;
mod viewmodel;

use model::Backend;
use model::ImageService;

/// We derive Deserialize/Serialize so we can persist app state on shutdown. (currently not used)
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct MyApp {
    #[serde(skip)]
    views: Vec<Box<dyn view::View>>,

    #[serde(skip)]
    image_service: Arc<ImageService>,

    #[serde(skip)]
    temp_backend: Arc<Backend>,
}

impl Default for MyApp {
    fn default() -> Self {
        let backend = Backend::new(); // TODO: inject backend into where it belongs
        backend.double(123); // TODO: remove test
        backend.double(11); // TODO: remove test

        let image_service = Arc::new(ImageService::new());

        let model_current = image_service.get_current_image();
        let model_preview = image_service.get_preview_image();

        let views: Vec<Box<dyn view::View>> = vec![
            Box::new(view::TopPanel::new(viewmodel::TopPanel::new(
                Arc::clone(&image_service),
                Arc::clone(&model_current),
                Arc::clone(&model_preview),
            ))),
            Box::new(view::CentralPanel::new(vec![
                Box::new(view::ImageFrame::new(viewmodel::ImageFrame::new(
                    "Current",
                    true,
                    Arc::clone(&image_service),
                    Arc::clone(&model_current),
                ))),
                Box::new(view::ImageFrame::new(viewmodel::ImageFrame::new(
                    "Preview",
                    false,
                    Arc::clone(&image_service),
                    Arc::clone(&model_preview),
                ))),
            ])),
        ];

        Self {
            views,
            image_service,
            temp_backend: Arc::new(backend),
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
}

impl eframe::App for MyApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
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
