use super::View;
use crate::app::viewmodel::top_panel::PropertyChangedNotification;
use crate::app::{modal, viewmodel};
use egui::{Context, Ui};
use rfd::FileHandle;
use tokio::sync::oneshot;

pub struct TopPanel {
    has_current: bool,
    has_preview: bool,

    rfd_promise: Option<oneshot::Receiver<Option<FileHandle>>>,

    viewmodel: viewmodel::TopPanel,
    vm_rx: tokio::sync::broadcast::Receiver<PropertyChangedNotification>,
}

impl TopPanel {
    pub fn new(viewmodel: viewmodel::TopPanel) -> Self {
        let vm_rx = viewmodel.get_receiver();

        Self {
            has_current: false,
            has_preview: false,
            rfd_promise: None,
            viewmodel,
            vm_rx,
        }
    }

    fn ui(&mut self, ui: &mut Ui) {
        egui::menu::bar(ui, |ui| {
            ui.menu_button("File", |ui| {
                if ui.button("open").clicked() {
                    self.rfd_promise = Some(modal::open_file_dialog());
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

        if let Some(rfd_promise) = &mut self.rfd_promise {
            if let Ok(file) = rfd_promise.try_recv() {
                self.viewmodel.open_file(file);
                self.rfd_promise.take();
            }
        }

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
