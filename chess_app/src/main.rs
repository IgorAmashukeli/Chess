

mod ui;
mod piece;
mod cell;
mod game;


fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Chess App",
        options,
        Box::new(|_cc| Ok(Box::new(ui::MyApp::default()))),
    )
}
