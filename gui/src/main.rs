mod app;
mod ui;

use app::FinanceApp;
use eframe::NativeOptions;

fn main() -> Result<(), eframe::Error> {
    let options = NativeOptions::default();
    eframe::run_native(
        "Finance",
        options,
        Box::new(|_cc| Ok(Box::new(FinanceApp::default()))),
    )
}
