use crate::app::FinanceApp;
use core::model::{Transaction, Category};

pub fn render(ui: &mut egui::Ui, app: &mut FinanceApp) {
    ui.label(
        egui::RichText::new("New Entry")
            .size(16.0)
            .color(egui::Color32::from_rgb(220, 220, 225))
            .strong()
    );
    ui.add_space(12.0);

    ui.horizontal(|ui| {
        ui.add(
            egui::TextEdit::singleline(&mut app.amount)
                .hint_text("Amount ($)")
                .desired_width(100.0)
        );

        egui::ComboBox::from_id_source("category_combo")
            .selected_text(&app.category)
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut app.category, "Food".to_string(), "Food");
                ui.selectable_value(&mut app.category, "Transport".to_string(), "Transport");
                ui.selectable_value(&mut app.category, "Leisure".to_string(), "Leisure");
                ui.selectable_value(&mut app.category, "Subscription".to_string(), "Subscription");
                ui.selectable_value(&mut app.category, "Salary".to_string(), "Salary");
                ui.selectable_value(&mut app.category, "Other".to_string(), "Other");
            });

        ui.add(
            egui::TextEdit::singleline(&mut app.description)
                .hint_text("Description")
                .desired_width(200.0)
        );

        ui.checkbox(&mut app.is_subscription, "Subscription");

        if ui.button("Add").clicked() {
            if let Ok(parsed_amount) = app.amount.trim().parse::<f64>() {
                if !app.description.trim().is_empty() {
                    let cat = match app.category.as_str() {
                        "Food" => Category::Food,
                        "Transport" => Category::Transport,
                        "Leisure" => Category::Leisure,
                        "Subscription" => Category::Subscription,
                        "Salary" => Category::Salary,
                        _ => Category::Other,
                    };

                    let new_transaction = Transaction {
                        id: chrono::Utc::now().timestamp_millis() as u64,
                        amount: parsed_amount,
                        description: app.description.clone(),
                        category: cat,
                        is_subscription: app.is_subscription,
                        date: chrono::Utc::now().naive_utc().date(),
                    };

                    app.add_transaction_and_save(new_transaction);

                    app.amount.clear();
                    app.description.clear();
                    app.is_subscription = false;
                }
            }
        }
    });
}
