use std::sync::Arc;

use eframe::Frame;
use egui::{ProgressBar, Ui};
use tokio::runtime::Runtime;

pub async fn test() {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([320.0, 140.0])
            .with_min_inner_size([320.0, 140.0])
            .with_transparent(false),
        vsync: true,
        hardware_acceleration: eframe::HardwareAcceleration::Preferred,
        renderer: eframe::Renderer::Glow,
        follow_system_theme: true,
        centered: false,
        ..Default::default()
    };
    eframe::run_native(
        "rustgrab - Uploading file",
        options,
        Box::new(|_cc| Box::from(ProgressDialog::from_theme(_cc.egui_ctx.style().visuals.dark_mode))),
    ).expect("failed to open!");
}
#[derive(Debug, Clone)]
struct ProgressDialog {
    pub runtime: Arc<Runtime>,
    pub is_dark: bool,

    pub min: i32,
    pub max: i32,
    pub value: i32,
    pub label_content: String,
    pub allow_close: bool
}

impl ProgressDialog {
    pub fn from_theme(theme: bool) -> Self {
        Self::default().set_theme(theme)
    }
    pub fn set_theme(&mut self, is_dark: bool) -> Self {
        self.is_dark = is_dark;
        self.clone()
    }
}

impl Default for ProgressDialog {
    fn default() -> Self {
        Self {
            runtime: Arc::new(Runtime::new().unwrap()),
            is_dark: true,

            min: 0,
            max: 100,
            value: 0,
            label_content: String::from("Running action..."),
            allow_close: true
        }
    }
}

impl eframe::App for ProgressDialog {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui| update_fn(self, ui));
    }
}
fn update_fn(value: &mut ProgressDialog, ui: &mut Ui) {
    ui.horizontal(|ui| {
        ui.label(value.label_content.as_str());
    });
    ui.horizontal(|ui| {
        let calc: f32 = ((value.value as f64 - value.min as f64) / value.max as f64) as f32;
        let pg = ProgressBar::new(calc);
        ui.add(pg);
    });
}