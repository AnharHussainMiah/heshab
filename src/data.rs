use crate::model;

async fn get_company_by_email(pool: &PgPool, email: &str) -> Result<(), sqlx::Error> {
    Ok(())
}

async fn search_customers(pool: &PgPool, name: &str, company_id: &i32) -> Result<(), sqlx::Error> {
    Ok(())
}

async fn get_customer_detail(pool: &PgPool, customer_id: &i32 , company_id: &i32) -> Result<(), sqlx::Error> {
    Ok(())
}

async fn get_customer_transactions(pool: &PgPool, customer_id: &i32, company_id: &i32) -> Result<(), sqlx::Error> {
    Ok(())
}

async fn add_customer_transaction(pool: &PgPool, customer_id: &i32, company_id: &i32, transaction: &i32) -> Result<(), sqlx::Error> {
    Ok(())
}

async fn delete_customer_transaction(pool: &PgPool, customer_id: &i32, company_id: &i32, transaction_id: &i32) -> Result<(), sqlx::Error> {
    Ok(())
}

async fn add_new_customer(pool: &PgPool, customer: &CustomerInfo) -> Result<(), sqlx::Error> {
    Ok(())
}

async fn update_customer(pool: &PgPool, customer: &CustomerInfo) -> Result<(), sqlx::Error> {
    Ok(())
}




