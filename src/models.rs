pub struct Company {
    id: i32,
    name: String,
    email: String,
    password: String
}

pub struct Customer {
    id: i32,
    company_id: i32,
    name: String,
    phone: String,
    address: String
}

pub struct CustomerInfo {
    company_id: i32,
    name: String,
    phone: String,
    address: String
}

pub struct Transactions {
    id: i32,
    amount: i32,
    timestamp: DateTime<Utc>
}