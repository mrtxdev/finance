use std::{fs::File, fs::read_to_string, io::{Error, Write}};
use serde::{Serialize, Deserialize};
use crate::model::Transaction;

#[derive(Serialize, Deserialize, Debug)]
pub struct FinanceManager {
    transactions: Vec<Transaction>,
}

impl FinanceManager {
    pub fn new(transactions: Vec<Transaction>) -> Self {
        Self { transactions }
    }

    pub fn add_transaction(&mut self, transaction: Transaction) {
        self.transactions.push(transaction);
    }

    pub fn get_all_transactions(&self) -> &Vec<Transaction> {
        &self.transactions
    }

    pub fn calculate_balance(&self) -> f64 {
        self.transactions.iter()
            .map(|t| t.amount)
            .sum()
    }

    pub fn get_subscriptions(&self) -> Vec<&Transaction> {
        self.transactions.iter()
            .filter(|t| t.is_subscription)
            .collect()
    }

    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }

    pub fn to_struct(file : &str) -> Result<FinanceManager, serde_json::Error> {
        serde_json::from_str(&file)
    }

    pub fn save_to_file(name: &str, content: &str) -> Result<(), Error> {
        let mut file = File::create(name)?;
        file.write_all(content.as_bytes())?;
        Ok(())
    }

    pub fn save_json_to_file(&self) -> Result<(), Box<dyn std::error::Error>> {
        Self::save_to_file("data.json", &self.to_json()?)?;
        Ok(())
    }

    pub fn load_from_file() -> Result<FinanceManager, Box<dyn std::error::Error>> {
        let file = read_to_string("data.json")?;
        Ok(Self::to_struct(&file)?)
    }

    pub fn remove(&mut self, id: u64) -> Result<(), Box<dyn std::error::Error>> {
        self.transactions.retain(|t| t.id != id);
        
        self.save_json_to_file()?;
        
        Ok(())
    }

}
