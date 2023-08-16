use serde::{Deserialize, Serialize};
use chrono::DateTime;
use chrono::Utc;

#[derive(Debug, Serialize)]
pub struct Company {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompanyInfo {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Customer {
    pub id: i32,
    pub company_id: i32,
    pub name: String,
    pub phone: String,
    pub address: String,
}

pub struct CustomerInfo {
    company_id: i32,
    name: String,
    phone: String,
    address: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListCustomerInfo {
    pub id: i32,
    pub name: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Transactions {
    pub id: i32,
    pub amount: i32,
    pub date_added: DateTime<Utc>
}
