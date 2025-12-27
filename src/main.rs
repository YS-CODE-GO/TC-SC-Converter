#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod converter;
mod file_ops;
mod queue;
mod app;

use eframe::egui;
use app::ConverterApp;
use image;

fn main() -> eframe::Result<()> {
    let mut native_options = eframe::NativeOptions::default();
    
    // Set window icon
    let icon_data = include_bytes!("../icon/icon.ico");
    if let Ok(image) = image::load_from_memory(icon_data) {
        let image = image.to_rgba8();
        let (width, height) = image.dimensions();
        native_options.viewport.icon = Some(Arc::new(egui::IconData {
            rgba: image.into_raw(),
            width,
            height,
        }));
    }

    native_options.viewport.title = Some("简繁转换工具 - By Yssssssss".to_owned());
    native_options.viewport.inner_size = Some(egui::vec2(1024.0, 780.0));
    native_options.viewport.min_inner_size = Some(egui::vec2(1024.0, 780.0));
    native_options.persist_window = true;

    eframe::run_native(
        "tc_sc_converter",
        native_options,
        Box::new(|cc| Ok(Box::new(ConverterApp::new(cc)))),
    )
}

use std::sync::Arc;
