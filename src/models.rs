use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct Company {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompanyInfo {
    id: i32,
    name: String,
}

pub struct Customer {
    id: i32,
    company_id: i32,
    name: String,
    phone: String,
    address: String,
}

pub struct CustomerInfo {
    company_id: i32,
    name: String,
    phone: String,
    address: String,
}

// pub struct Transactions {
//     id: i32,
//     customer_id: i32,
//     amount: i32,
//     date_added: DateTime<Utc>
// }
