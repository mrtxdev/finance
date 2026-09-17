use core::finance::{FinanceManager};
use core::model::{Transaction, Category};
use std::assert_eq;
use chrono::NaiveDate;

#[test]
fn test_calculate_balance() {
    let transaction = vec![
        Transaction {
            id: 1,
            amount: 1500.0,
            description: String::from("Monthly salary"),
            category: Category::Salary,
            is_subscription: false,
            date: NaiveDate::from_ymd_opt(2026, 9, 1).unwrap(),
        },
        Transaction {
            id: 2,
            amount: -50.0,
            description: String::from("Sunday lunch"),
            category: Category::Food,
            is_subscription: false,
            date: NaiveDate::from_ymd_opt(2026, 9, 5).unwrap(),
        },
        Transaction {
            id: 3,
            amount: -35.0,
            description: String::from("Streaming subscription"),
            category: Category::Subscription,
            is_subscription: true,
            date: NaiveDate::from_ymd_opt(2026, 9, 10).unwrap(),
        },
    ];

    let finance_manager = FinanceManager::new(transaction);
    let calcule = finance_manager.calculate_balance(); 
    assert_eq!(calcule, 1415.);
}

#[test]
fn test_filter_subscriptions() {
    let transaction = vec![
        Transaction {
            id: 1,
            amount: 1500.0,
            description: String::from("Monthly salary"),
            category: Category::Salary,
            is_subscription: false,
            date: NaiveDate::from_ymd_opt(2026, 9, 1).unwrap(),
        },
        Transaction {
            id: 2,
            amount: -50.0,
            description: String::from("Sunday lunch"),
            category: Category::Food,
            is_subscription: false,
            date: NaiveDate::from_ymd_opt(2026, 9, 5).unwrap(),
        },
        Transaction {
            id: 3,
            amount: -35.0,
            description: String::from("Streaming subscription"),
            category: Category::Subscription,
            is_subscription: true,
            date: NaiveDate::from_ymd_opt(2026, 9, 10).unwrap(),
        },
    ];
    
    let finance = FinanceManager::new(transaction.clone());
    let filter = finance.get_subscriptions();
    
    assert_eq!(filter.len(), 1);
    
    assert_eq!(filter[0].is_subscription, transaction[2].is_subscription);
}

#[test]
fn test_save_json_to_file() {
    let transactions = vec![
        Transaction {
            id: 1,
            amount: 2500.0,
            description: String::from("Freelance project payment"),
            category: Category::Salary,
            is_subscription: false,
            date: NaiveDate::from_ymd_opt(2026, 9, 1).unwrap(),
        },
        Transaction {
            id: 2,
            amount: -12.50,
            description: String::from("Morning coffee"),
            category: Category::Food,
            is_subscription: false,
            date: NaiveDate::from_ymd_opt(2026, 9, 2).unwrap(),
        },
        Transaction {
            id: 3,
            amount: -45.0,
            description: String::from("Gym membership"),
            category: Category::Subscription,
            is_subscription: true,
            date: NaiveDate::from_ymd_opt(2026, 9, 3).unwrap(),
        },
        Transaction {
            id: 4,
            amount: -80.0,
            description: String::from("Gasoline for the car"),
            category: Category::Transport,
            is_subscription: false,
            date: NaiveDate::from_ymd_opt(2026, 9, 4).unwrap(),
        },
        Transaction {
            id: 5,
            amount: -15.0,
            description: String::from("Cloud storage plan"),
            category: Category::Subscription,
            is_subscription: true,
            date: NaiveDate::from_ymd_opt(2026, 9, 5).unwrap(),
        },
    ];
    let finance = FinanceManager::new(transactions);
    let result = finance.save_json_to_file();
    assert!(result.is_ok());
}

#[test]
fn test_load_from_file() {
    let transactions = vec![
        Transaction {
            id: 1,
            amount: 2500.0,
            description: String::from("Freelance project payment"),
            category: Category::Salary,
            is_subscription: false,
            date: NaiveDate::from_ymd_opt(2026, 9, 1).unwrap(),
        },
        Transaction {
            id: 2,
            amount: -12.50,
            description: String::from("Morning coffee"),
            category: Category::Food,
            is_subscription: false,
            date: NaiveDate::from_ymd_opt(2026, 9, 2).unwrap(),
        },
        Transaction {
            id: 3,
            amount: -45.0,
            description: String::from("Gym membership"),
            category: Category::Subscription,
            is_subscription: true,
            date: NaiveDate::from_ymd_opt(2026, 9, 3).unwrap(),
        },
        Transaction {
            id: 4,
            amount: -80.0,
            description: String::from("Gasoline for the car"),
            category: Category::Transport,
            is_subscription: false,
            date: NaiveDate::from_ymd_opt(2026, 9, 4).unwrap(),
        },
        Transaction {
            id: 5,
            amount: -15.0,
            description: String::from("Cloud storage plan"),
            category: Category::Subscription,
            is_subscription: true,
            date: NaiveDate::from_ymd_opt(2026, 9, 5).unwrap(),
        },
    ];
    let result = FinanceManager::load_from_file();
    assert!(result.is_ok());
}

#[test]
fn test_remove_transaction() {
    let transactions = vec![
        Transaction {
            id: 1,
            amount: 100.0,
            description: String::from("Item one"),
            category: Category::Other,
            is_subscription: false,
            date: NaiveDate::from_ymd_opt(2026, 9, 1).unwrap(),
        },
        Transaction {
            id: 2,
            amount: -30.0,
            description: String::from("Item to remove"),
            category: Category::Food,
            is_subscription: false,
            date: NaiveDate::from_ymd_opt(2026, 9, 2).unwrap(),
        },
        Transaction {
            id: 3,
            amount: -20.0,
            description: String::from("Item three"),
            category: Category::Subscription,
            is_subscription: true,
            date: NaiveDate::from_ymd_opt(2026, 9, 3).unwrap(),
        },
    ];

    let mut finance_manager = FinanceManager::new(transactions);
    
    let result = finance_manager.remove(2);
    assert!(result.is_ok());

    let remaining = finance_manager.get_all_transactions();
    assert_eq!(remaining.len(), 2);

    assert_eq!(remaining[0].id, 1);
    assert_eq!(remaining[1].id, 3);

    let balance = finance_manager.calculate_balance();
    assert_eq!(balance, 80.0);
}
