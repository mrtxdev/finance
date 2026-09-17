use serde::{Serialize, Deserialize};
use chrono::NaiveDate;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Category {
    Food,
    Transport,
    Leisure,
    Subscription,
    Salary,
    Other,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Transaction {
    pub id: u64,
    pub amount: f64,
    pub description: String,
    pub category: Category,
    pub is_subscription: bool, 
    
    #[serde(with = "crate::serializers")]
    pub date: NaiveDate,
}
