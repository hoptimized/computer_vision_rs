use egui_extras::RetainedImage;
use std::future::Future;

pub enum Message {
    ImageLoaded(RetainedImage),
}

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct MyApp {
    #[serde(skip)]
    message_channel: (
        std::sync::mpsc::Sender<Message>,
        std::sync::mpsc::Receiver<Message>,
    ),

    #[serde(skip)]
    image: Option<RetainedImage>,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            message_channel: std::sync::mpsc::channel(),
            image: None,
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
        let Self { image, .. } = self;

        ctx.request_repaint();

        while let Ok(message) = self.message_channel.1.try_recv() {
            match message {
                Message::ImageLoaded(new_image) => {
                    image.replace(new_image);
                }
            };
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            let open_button = ui.add(egui::widgets::Button::new("Open Image"));

            if open_button.clicked() {
                let task = rfd::AsyncFileDialog::new()
                    .add_filter("Image files", &["png", "jpg", "jpeg"])
                    .set_directory("/")
                    .pick_file();

                let message_sender = self.message_channel.0.clone();

                execute(async move {
                    let file = task.await;

                    // TODO: reduce nesting

                    if let Some(file) = file {
                        let data = file.read().await;

                        let new_image = RetainedImage::from_image_bytes("foo", &*data);
                        if let Ok(new_image) = new_image {
                            message_sender.send(Message::ImageLoaded(new_image)).ok();
                        }
                    }
                });
            }

            if let Some(image) = image {
                image.show(ui);
            }
        });
    }
}

// TODO: check if this is how multithreading should be done:

#[cfg(not(target_arch = "wasm32"))]
fn execute<F: Future<Output = ()> + Send + 'static>(f: F) {
    std::thread::spawn(move || futures::executor::block_on(f));
}
#[cfg(target_arch = "wasm32")]
fn execute<F: Future<Output = ()> + 'static>(f: F) {
    wasm_bindgen_futures::spawn_local(f);
}
