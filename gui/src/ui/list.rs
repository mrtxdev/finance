use core::finance::FinanceManager;

pub fn render(ui: &mut egui::Ui, finance_manager: &mut FinanceManager) {
    ui.label(
        egui::RichText::new("Recent Entries")
            .size(16.0)
            .color(egui::Color32::from_rgb(220, 220, 225))
            .strong()
    );
    ui.add_space(12.0);

    egui::Frame::none()
        .fill(egui::Color32::from_rgb(24, 24, 28))
        .rounding(8.0)
        .inner_margin(16.0)
        .show(ui, |ui| {
            let mut id_to_remove = None;

            for transaction in finance_manager.get_all_transactions() {
                ui.horizontal(|ui| {
                    if ui.button("🗑").clicked() {
                        id_to_remove = Some(transaction.id);
                    }
                    ui.add_space(8.0);

                    ui.label(egui::RichText::new(transaction.date.format("%d/%m/%Y").to_string()).color(egui::Color32::from_rgb(130, 130, 140)));
                    ui.add_space(16.0);
                    ui.label(egui::RichText::new(&transaction.description).color(egui::Color32::from_rgb(220, 220, 225)));
                    ui.add_space(8.0);
                    ui.label(
                        egui::RichText::new(format!("{:?}", transaction.category))
                            .size(11.0)
                            .color(egui::Color32::from_rgb(150, 150, 165))
                    );
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        let color = if transaction.amount >= 0.0 {
                            egui::Color32::from_rgb(80, 200, 120)
                        } else {
                            egui::Color32::from_rgb(225, 100, 100)
                        };
                        let prefix = if transaction.amount >= 0.0 { "+" } else { "" };
                        ui.label(egui::RichText::new(format!("{}{:.2}", prefix, transaction.amount)).color(color).strong());
                    });
                });
                ui.add_space(8.0);
            }

            if let Some(id) = id_to_remove {
                let _ = finance_manager.remove(id);
            }
        });
}
