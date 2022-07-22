pub mod central_panel;
pub mod image_frame;
pub mod top_panel;

pub use central_panel::CentralPanel;
pub use image_frame::ImageFrame;
pub use top_panel::TopPanel;

pub trait View {
    fn show(&mut self, ctx: &egui::Context);
}
