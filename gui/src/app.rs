use crate::ui::Tab;
use core::finance::FinanceManager;
use core::model::Transaction;

pub struct FinanceApp {
    pub current_tab: Tab,
    pub amount: String,
    pub category: String,
    pub description: String,
    pub is_subscription: bool,
    finance_manager: FinanceManager,
}

impl Default for FinanceApp {
    fn default() -> Self {
        let manager = match FinanceManager::load_from_file() {
            Ok(m) => m,
            Err(_) => FinanceManager::new(vec![]),
        };

        Self {
            current_tab: Tab::Entries,
            amount: String::new(),
            category: "Food".to_string(),
            description: String::new(),
            is_subscription: false,
            finance_manager: manager,
        }
    }
}

impl FinanceApp {
    pub fn get_finance_manager(&self) -> &FinanceManager {
        &self.finance_manager
    }

    pub fn add_transaction_and_save(&mut self, transaction: Transaction) {
        self.finance_manager.add_transaction(transaction);
        let _ = self.finance_manager.save_json_to_file();
    }
}

impl eframe::App for FinanceApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let mut style = (*ctx.style()).clone();
        style.visuals = egui::Visuals::dark();
        style.visuals.window_fill = egui::Color32::from_rgb(18, 18, 20);
        style.visuals.panel_fill = egui::Color32::from_rgb(18, 18, 20);
        style.visuals.widgets.noninteractive.bg_fill = egui::Color32::from_rgb(26, 26, 30);
        style.visuals.widgets.inactive.bg_fill = egui::Color32::from_rgb(34, 34, 39);
        style.visuals.widgets.hovered.bg_fill = egui::Color32::from_rgb(45, 45, 52);
        style.visuals.widgets.active.bg_fill = egui::Color32::from_rgb(55, 55, 64);
        ctx.set_style(style);

        egui::TopBottomPanel::top("top_panel")
            .frame(
                egui::Frame::new()
                    .fill(egui::Color32::from_rgb(24, 24, 28))
                    .inner_margin(egui::Margin {
                        left: 16,
                        right: 16,
                        top: 12,
                        bottom: 12,
                    })
            )
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.heading(
                        egui::RichText::new("Finance")
                            .color(egui::Color32::from_rgb(240, 240, 245))
                            .strong()
                    );
                    ui.add_space(32.0);
                    ui.selectable_value(&mut self.current_tab, Tab::Entries, "Entries");
                    ui.add_space(8.0);
                    ui.selectable_value(&mut self.current_tab, Tab::Dashboard, "Dashboard");
                });
            });

        egui::CentralPanel::default()
            .frame(
                egui::Frame::new()
                    .fill(egui::Color32::from_rgb(18, 18, 20))
                    .inner_margin(egui::Margin::same(24))
            )
            .show(ctx, |ui| {
                match self.current_tab {
                    Tab::Entries => {
                        ui.vertical(|ui| {
                            crate::ui::form::render(ui, self);
                            ui.add_space(24.0);
                            crate::ui::list::render(ui, &mut self.finance_manager);
                        });
                    }
                    Tab::Dashboard => {
                        crate::ui::dashboard::render(ui, &self.finance_manager);
                    }
                }
            });
    }
}
