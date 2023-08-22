use crate::models::Company;
use crate::models::Customer;
use crate::models::Transactions;
use crate::models::CustomerInfo;
use crate::models::ListCustomerInfo;
use crate::customer::CustomerPayload;
use sqlx::PgPool;
use uuid::Uuid;
use chrono::NaiveDateTime;
use chrono::Utc;
use chrono::DateTime;

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
        select id, name, phone from customer where company_id = $1 and lower(name) ~ $2;
        "#,
        company_id,
        name)
        .fetch_all(pool)
        .await?;
    
    Ok(rec.into_iter().map(|row| ListCustomerInfo {
        id: row.id,
        name: row.name.unwrap_or("".to_string()),
        phone: row.phone.unwrap_or("".to_string())
    }).collect::<Vec<ListCustomerInfo>>())
}

pub async fn get_customer_detail(
    pool: &PgPool,
    customer_id: &i32,
    company_id: &i32,
) -> Result<Customer, sqlx::Error> {
    let rec = sqlx::query!(
        r#"
        select id, company_id, name, phone, address from customer where company_id = $1 and id = $2;
        "#,
        company_id,
        customer_id
    )
    .fetch_one(pool)
    .await?;
    Ok(Customer {
        id: rec.id,
        company_id: rec.company_id.unwrap_or(0),
        name: rec.name.unwrap_or("".to_string()),
        phone: rec.phone.unwrap_or("".to_string()),
        address: rec.address.unwrap_or("".to_string())
    })
}

pub async fn get_customer_transactions(
    pool: &PgPool,
    customer_id: &i32,
    company_id: &i32,
) -> Result<Vec<Transactions>, sqlx::Error> {
    let rec = sqlx::query!(
        r#"
        select id, amount, date_added from customer_transactions where company_id = $1 and customer_id = $2;
        "#,
        company_id,
        customer_id)
        .fetch_all(pool)
        .await?;
    
    Ok(rec.into_iter().map(|row| Transactions {
        id: row.id,
        amount: row.amount.unwrap_or(0),
        date_added: DateTime::from_utc(
            row.date_added.unwrap_or(Utc::now().naive_utc()),
            Utc
        )
    }).collect::<Vec<Transactions>>())
}

pub async fn add_customer_transaction(
    pool: &PgPool,
    customer_id: &i32,
    company_id: &i32,
    amount: &i32,
) -> Result<(), sqlx::Error> {
    let _ = sqlx::query!(
        r#"
        insert into customer_transactions (customer_id, company_id, amount, date_added)
                                   values ($1, $2, $3, now());
        "#,
        customer_id,
        company_id,
        amount
    )
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn delete_customer_transaction(
    pool: &PgPool,
    company_id: &i32,
    transaction_id: &i32,
) -> Result<(), sqlx::Error> {
    let _ = sqlx::query!(
        r#"
        delete from customer_transactions where company_id = $1 and id = $2;
        "#,
        company_id,
        transaction_id
    )
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn add_new_customer(
    pool: &PgPool, 
    company_id: &i32,
    customer: &CustomerPayload
) -> Result<(), sqlx::Error> {
    let _ = sqlx::query!(
        r#"
        insert into customer (company_id ,name ,phone ,address)
                    values   ($1         ,$2   ,$3    ,$4     );
        "#,
        company_id,
        customer.name,
        customer.phone,
        customer.address
    )
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn update_customer(
    pool: &PgPool,
    company_id: &i32,
    customer: &CustomerPayload
) -> Result<(), sqlx::Error> {
    let _ = sqlx::query!(
        r#"
        update customer
            set
                name = $3,
                phone = $4,
                address = $5
            where
                company_id = $1
                and
                id = $2
        "#,
        company_id,
        customer.customer_id,
        customer.name,
        customer.phone,
        customer.address
    )
    .execute(pool)
    .await?;

    Ok(())
}
