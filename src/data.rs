use crate::models::Company;
use crate::models::CustomerInfo;
use crate::models::ListCustomerInfo;
use sqlx::PgPool;
use uuid::Uuid;

pub async fn get_company_by_email(pool: &PgPool, email: &str) -> Result<Company, sqlx::Error> {
    let rec = sqlx::query!(
        r#"
        select * from company where lower(trim(email)) = $1
        "#,
        email
    )
    .fetch_one(pool)
    .await?;

    Ok(Company {
        id: rec.id,
        name: rec.name.unwrap_or("".to_string()),
        email: rec.email.unwrap_or("".to_string()),
        password: rec.password.unwrap_or(Uuid::new_v4().to_string()),
    })
}

pub async fn search_customers(pool: &PgPool, name: &str, company_id: &i32) -> Result<Vec<ListCustomerInfo>, sqlx::Error> {
    let rec = sqlx::query!(
        r#"
        select id, name from customer where company_id = $1 and lower(name) ~ $2;
        "#,
        company_id,
        name)
        .fetch_all(pool)
        .await?;
    
    Ok(rec.into_iter().map(|row| ListCustomerInfo {
        id: row.id,
        name: row.name.unwrap_or("".to_string())
    }).collect::<Vec<ListCustomerInfo>>())
}

async fn get_customer_detail(
    pool: &PgPool,
    customer_id: &i32,
    company_id: &i32,
) -> Result<(), sqlx::Error> {
    Ok(())
}

async fn get_customer_transactions(
    pool: &PgPool,
    customer_id: &i32,
    company_id: &i32,
) -> Result<(), sqlx::Error> {
    Ok(())
}

async fn add_customer_transaction(
    pool: &PgPool,
    customer_id: &i32,
    company_id: &i32,
    transaction: &i32,
) -> Result<(), sqlx::Error> {
    Ok(())
}

async fn delete_customer_transaction(
    pool: &PgPool,
    customer_id: &i32,
    company_id: &i32,
    transaction_id: &i32,
) -> Result<(), sqlx::Error> {
    Ok(())
}

async fn add_new_customer(pool: &PgPool, customer: &CustomerInfo) -> Result<(), sqlx::Error> {
    Ok(())
}

async fn update_customer(pool: &PgPool, customer: &CustomerInfo) -> Result<(), sqlx::Error> {
    Ok(())
}
