#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eframe::egui;
use truenas_client::app;
fn main() -> eframe::Result 
{

    let options = eframe::NativeOptions
    {
        viewport: egui::ViewportBuilder::default()
        //.with_fullscreen(true)
        .with_title("TrueNAS API Client")
        .with_icon(
            eframe::icon_data::from_png_bytes(&include_bytes!("../icon.png")[..]).unwrap()
        ),
        ..Default::default()
    };

    eframe::run_native(
        "TrueNAS API Client", 
        options, 
        Box::new(|cc|
            {
                egui_extras::install_image_loaders(&cc.egui_ctx);
                Ok(Box::new(app::Dashboard::new(cc)))
            }),
    )
}
