use super::View;

pub struct CentralPanel {
    children: Vec<Box<dyn View>>,
}

impl CentralPanel {
    pub fn new(children: Vec<Box<dyn View>>) -> Self {
        Self { children }
    }
}

impl View for CentralPanel {
    fn show(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |_ui| {
            for view in self.children.iter_mut() {
                view.show(ctx);
            }
        });
    }
}
