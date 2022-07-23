use arc_swap::ArcSwap;
use image::DynamicImage;
use std::sync::Arc;
use tokio::sync::broadcast;

pub struct Image {
    data: ArcSwap<Option<DynamicImage>>,
    channel: (broadcast::Sender<()>, broadcast::Receiver<()>),
}

impl Image {
    pub fn new() -> Self {
        Self {
            data: ArcSwap::from(Arc::new(None)),
            channel: broadcast::channel(32),
        }
    }

    #[allow(dead_code)]
    pub fn get(&self) -> Arc<Option<DynamicImage>> {
        self.data.load().clone()
    }

    #[allow(dead_code)]
    pub fn set(&self, data: Option<DynamicImage>) {
        self.data.swap(Arc::new(data));
        self.notify_property_changed();
    }

    #[allow(dead_code)]
    pub fn set_arc(&self, data: Arc<Option<DynamicImage>>) {
        self.data.swap(data);
        self.notify_property_changed();
    }

    #[allow(dead_code)]
    pub fn take(&self) -> Arc<Option<DynamicImage>> {
        let new_data = self.data.swap(Arc::new(None));
        self.notify_property_changed();
        new_data
    }

    #[allow(dead_code)]
    pub fn get_property_changed_rx(&self) -> broadcast::Receiver<()> {
        self.channel.0.subscribe()
    }

    fn notify_property_changed(&self) {
        self.channel.0.send(()).ok();
    }
}
