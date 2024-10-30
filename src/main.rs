use chrono::Local;
use eframe::egui;
use egui::viewport::{self, ViewportBuilder};

fn main() {
    let native_options = eframe::NativeOptions {
        viewport: ViewportBuilder::default().with_decorations(true),
        ..Default::default()
    };
    eframe::run_native(
        "my_app",
        native_options,
        Box::new(|cc| Ok(Box::new(MyEguiApp::new(cc)))),
    );
}

#[derive(Default)]
struct MyEguiApp {}

impl MyEguiApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        Self::default()
    }
}

impl eframe::App for MyEguiApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        let fmt = "%Y:%M:%D %H:%M:%S";
        let now = Local::now().format(fmt);
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading(now.to_string());
        });
    }
}
