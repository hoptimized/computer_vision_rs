use crate::app::{GuiAction, View};
use egui::{Context, Ui};
use std::sync::mpsc;

pub struct TopPanel {
    tx: mpsc::Sender<GuiAction>,
    has_current: bool,
    has_preview: bool,
}

impl TopPanel {
    pub fn new(tx: mpsc::Sender<GuiAction>) -> Self {
        Self {
            tx,
            has_current: false,
            has_preview: false,
        }
    }

    pub fn set_has_current(&mut self, has_current: bool) {
        self.has_current = has_current;
    }

    pub fn set_has_preview(&mut self, has_preview: bool) {
        self.has_preview = has_preview;
    }
}

impl View for TopPanel {
    fn show(&mut self, ctx: &Context) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            self.ui(ui);
        });
    }

    fn ui(&mut self, ui: &mut Ui) {
        egui::menu::bar(ui, |ui| {
            ui.menu_button("File", |ui| {
                if ui.button("open").clicked() {
                    self.tx.send(GuiAction::OpenFileDialog).ok();
                }

                ui.separator();

                if ui
                    .add_enabled(self.has_current, egui::Button::new("grayscale"))
                    .clicked()
                {
                    self.tx.send(GuiAction::ApplyGrayscale).ok();
                }

                if ui
                    .add_enabled(self.has_current, egui::Button::new("invert"))
                    .clicked()
                {
                    self.tx.send(GuiAction::ApplyInvert).ok();
                }

                ui.separator();

                if ui
                    .add_enabled(self.has_preview, egui::Button::new("accept"))
                    .clicked()
                {
                    self.tx.send(GuiAction::AcceptOperation).ok();
                }

                if ui
                    .add_enabled(self.has_preview, egui::Button::new("discard"))
                    .clicked()
                {
                    self.tx.send(GuiAction::DiscardOperation).ok();
                }

                ui.separator();

                if ui
                    .add_enabled(
                        self.has_current || self.has_preview,
                        egui::Button::new("reset"),
                    )
                    .clicked()
                {
                    self.tx.send(GuiAction::ResetAll).ok();
                }
            });
        });
    }
}
