use super::View;
use crate::app::viewmodel;
use crate::app::viewmodel::top_panel::PropertyChangedNotification;
use egui::{Context, Ui};

pub struct TopPanel {
    has_current: bool,
    has_preview: bool,

    viewmodel: viewmodel::TopPanel,
    vm_rx: tokio::sync::broadcast::Receiver<PropertyChangedNotification>,
}

impl TopPanel {
    pub fn new(viewmodel: viewmodel::TopPanel) -> Self {
        let vm_rx = viewmodel.get_receiver();

        Self {
            has_current: false,
            has_preview: false,
            viewmodel,
            vm_rx,
        }
    }

    fn ui(&mut self, ui: &mut Ui) {
        egui::menu::bar(ui, |ui| {
            ui.menu_button("File", |ui| {
                if ui.button("open").clicked() {
                    self.viewmodel.open_file_dialog();
                }

                ui.separator();

                if ui
                    .add_enabled(self.has_current, egui::Button::new("grayscale"))
                    .clicked()
                {
                    self.viewmodel.apply_grayscale();
                }

                if ui
                    .add_enabled(self.has_current, egui::Button::new("invert"))
                    .clicked()
                {
                    self.viewmodel.apply_invert();
                }

                ui.separator();

                if ui
                    .add_enabled(self.has_preview, egui::Button::new("accept"))
                    .clicked()
                {
                    self.viewmodel.accept_operation();
                }

                if ui
                    .add_enabled(self.has_preview, egui::Button::new("discard"))
                    .clicked()
                {
                    self.viewmodel.discard_operation();
                }

                ui.separator();

                if ui
                    .add_enabled(
                        self.has_current || self.has_preview,
                        egui::Button::new("reset"),
                    )
                    .clicked()
                {
                    self.viewmodel.reset_images();
                }
            });
        });
    }
}

impl View for TopPanel {
    fn show(&mut self, ctx: &Context) {
        self.viewmodel.process_messages();

        while let Ok(notification) = self.vm_rx.try_recv() {
            match notification {
                PropertyChangedNotification::HasCurrent => {
                    self.has_current = self.viewmodel.get_has_current()
                }
                PropertyChangedNotification::HasPreview => {
                    self.has_preview = self.viewmodel.get_has_preview()
                }
            }
        }

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            self.ui(ui);
        });
    }
}
