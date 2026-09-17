use core::finance::FinanceManager;

pub fn render(ui: &mut egui::Ui, finance_manager: &FinanceManager) {
    ui.heading(
        egui::RichText::new("Dashboard")
            .color(egui::Color32::from_rgb(240, 240, 245))
    );
    ui.add_space(16.0);

    ui.horizontal(|ui| {
        let card_size = egui::vec2(200.0, 90.0);

        let render_card = |ui: &mut egui::Ui, title: &str, value: String| {
            egui::Frame::none()
                .fill(egui::Color32::from_rgb(24, 24, 28))
                .rounding(8.0)
                .inner_margin(16.0)
                .show(ui, |ui| {
                    ui.set_min_size(card_size);
                    ui.vertical(|ui| {
                        ui.label(egui::RichText::new(title).size(12.0).color(egui::Color32::from_rgb(140, 140, 150)));
                        ui.add_space(8.0);
                        ui.label(egui::RichText::new(value).size(20.0).color(egui::Color32::from_rgb(240, 240, 245)).strong());
                    });
                });
        };

        let balance = finance_manager.calculate_balance();
        let balance_text = format!("${:.2}", balance);
        
        let subscriptions = finance_manager.get_subscriptions();
        let subs_total: f64 = subscriptions.iter().map(|t| t.amount).sum();

        render_card(ui, "Total Balance", balance_text);
        ui.add_space(12.0);
        render_card(ui, "Subscriptions Total", format!("${:.2}", subs_total.abs()));
        ui.add_space(12.0);
        render_card(ui, "Active Subscriptions", format!("{}", subscriptions.len()));
    });

    ui.add_space(32.0);

    ui.label(
        egui::RichText::new("Spending Distribution By Category")
            .size(16.0)
            .color(egui::Color32::from_rgb(220, 220, 225))
            .strong()
    );
    ui.add_space(16.0);

    egui::Frame::none()
        .fill(egui::Color32::from_rgb(24, 24, 28))
        .rounding(8.0)
        .inner_margin(16.0)
        .show(ui, |ui| {
            let categories = [
                ("Food", 0.15),
                ("Transport", 0.45),
                ("Leisure", 0.05),
                ("Subscription", 0.30),
            ];

            for (cat, val) in categories {
                ui.horizontal(|ui| {
                    ui.add(egui::Label::new(egui::RichText::new(cat).color(egui::Color32::from_rgb(140, 140, 150))).sense(egui::Sense::hover()));
                    ui.add_space(16.0);
                    let bar = egui::ProgressBar::new(val as f32)
                        .fill(egui::Color32::from_rgb(90, 110, 240))
                        .desired_height(12.0);
                    ui.add(bar);
                });
                ui.add_space(8.0);
            }
        });
}
