use eframe::egui;

use crate::game::*;

pub struct MyApp {
    game: Game,
}

impl Default for MyApp {
    fn default() -> Self { Self { game: Game::default() } }
}

impl eframe::App for MyApp {
    /*fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        egui::Color32::DARK_BLUE.to_normalized_gamma_f32()
    }*/

    

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
    egui::TopBottomPanel::top("top")
    .resizable(true)
    .default_height(40.0)
    .show(ctx, |ui| {
        ui.label("Top");
    });
    
    egui::TopBottomPanel::bottom("bottom")
    .resizable(true)
    .default_height(30.0)
    .show(ctx, |ui| {
        ui.label("Bottom");
    });
    
    egui::SidePanel::left("left")
    .resizable(true)
    .default_width(200.0) 
    .show(ctx, |ui| {
        ui.label("Left");
    });
    
    egui::SidePanel::right("right")
    .resizable(true)
    .default_width(150.0)
    .show(ctx, |ui| {
        ui.label("Right");
    });
    
    egui::CentralPanel::default()
    .show(ctx, |ui| {
        ui.label("Central");
    });
}

}

