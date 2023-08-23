use crate::models::Transactions;
use crate::CompanyInfo;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use warp::http::StatusCode;
use warp::reply::{json, with_status};

#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionPayload {
    pub customer_id: i32,
    pub amount: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CustomerPayload {
    pub customer_id: i32,
    pub name: String,
    pub phone: String,
    pub address: String,
}

pub async fn search_customers_by_name(
    query: String,
    company: CompanyInfo,
    pool: PgPool,
) -> Result<impl warp::Reply, warp::Rejection> {
    println!("==> searching customer by name [{}..]", query);

    if let Ok(customers) = crate::data::search_customers(&pool, &query, &company.id).await {
        return Ok(with_status(json(&customers), StatusCode::OK));
    }

    Ok(with_status(json(&"No Data"), StatusCode::NOT_FOUND))
}

pub async fn get_customer_detail(
    customer_id: i32,
    company: CompanyInfo,
    pool: PgPool,
) -> Result<impl warp::Reply, warp::Rejection> {
    println!("==> getting customer detail for ID: {}", customer_id);
    if let Ok(customer) = crate::data::get_customer_detail(&pool, &customer_id, &company.id).await {
        return Ok(with_status(json(&customer), StatusCode::OK));
    }

    Ok(with_status(json(&"No Data"), StatusCode::NOT_FOUND))
}

pub async fn get_customer_transactions(
    customer_id: i32,
    company: CompanyInfo,
    pool: PgPool,
) -> Result<impl warp::Reply, warp::Rejection> {
    println!(
        "==> getting customer transactions for customer ID: {}",
        customer_id
    );
    if let Ok(transactions) =
        crate::data::get_customer_transactions(&pool, &customer_id, &company.id).await
    {
        return Ok(with_status(json(&transactions), StatusCode::OK));
    }

    Ok(with_status(json(&"No Data"), StatusCode::NOT_FOUND))
}

pub async fn add_customer_transaction(
    company: CompanyInfo,
    payload: TransactionPayload,
    pool: PgPool,
) -> Result<impl warp::Reply, warp::Rejection> {
    println!(
        "==> adding new customer transaction for customer ID: {}",
        payload.customer_id
    );
    match crate::data::add_customer_transaction(
        &pool,
        &payload.customer_id,
        &company.id,
        &payload.amount,
    )
    .await
    {
        Ok(_) => {
            return Ok(with_status(json(&"success".to_string()), StatusCode::OK));
        }
        Err(e) => Ok(with_status(
            json(&format!("Unable to add customer transaction: {}", e)),
            StatusCode::INTERNAL_SERVER_ERROR,
        )),
    }
}

pub async fn delete_customer_transaction(
    transaction_id: i32,
    company: CompanyInfo,
    pool: PgPool,
) -> Result<impl warp::Reply, warp::Rejection> {
    println!(
        "==> deleting customer transaction with tx ID: {}",
        transaction_id
    );
    if let Ok(_) =
        crate::data::delete_customer_transaction(&pool, &company.id, &transaction_id).await
    {
        return Ok(with_status(json(&"success".to_string()), StatusCode::OK));
    }

    Ok(with_status(
        json(&"Unable to delete customer transaction"),
        StatusCode::INTERNAL_SERVER_ERROR,
    ))
}

pub async fn add_new_customer(
    company: CompanyInfo,
    customer: CustomerPayload,
    pool: PgPool,
) -> Result<impl warp::Reply, warp::Rejection> {
    println!("==> adding new customer [{}...]", customer.name);
    if let Some(error) = self::validate_cusomter_error(&customer) {
        return Ok(with_status(json(&error), StatusCode::BAD_REQUEST));
    }

    if let Ok(_) = crate::data::add_new_customer(&pool, &company.id, &customer).await {
        return Ok(with_status(json(&"success".to_string()), StatusCode::OK));
    }

    Ok(with_status(
        json(&"Unable to delete customer transaction"),
        StatusCode::INTERNAL_SERVER_ERROR,
    ))
}

pub async fn update_new_customer(
    company: CompanyInfo,
    customer: CustomerPayload,
    pool: PgPool,
) -> Result<impl warp::Reply, warp::Rejection> {
    println!(
        "==> attempting to update customer ID [{}]",
        customer.customer_id
    );
    if let Some(error) = self::validate_cusomter_error(&customer) {
        return Ok(with_status(json(&error), StatusCode::BAD_REQUEST));
    }

    if let Ok(_) = crate::data::update_customer(&pool, &company.id, &customer).await {
        return Ok(with_status(json(&"success".to_string()), StatusCode::OK));
    }

    Ok(with_status(
        json(&"Unable to delete customer transaction"),
        StatusCode::INTERNAL_SERVER_ERROR,
    ))
}

fn validate_cusomter_error(customer: &CustomerPayload) -> Option<String> {
    /*
        some minimum checks?
        name is max 100 characters
        phone is all numbers and only 11 digits
        address is max 200 characters
    */

    if customer.name == "" {
        return Some("Sorry the customer's name can not be empty".to_string());
    }

    if customer.phone == "" {
        return Some("Sorry the customer's phone can not be empty".to_string());
    }

    if customer.name.len() > 50 {
        return Some("Sorry the customer's name can not be longer then 50 characters".to_string());
    }

    if customer.phone.len() > 11 {
        return Some("Sorry the customer's phone can only have 11 digits".to_string());
    }

    if customer.address.len() > 100 {
        return Some(
            "Sorry the customer's addres can not be longer then 100 characters".to_string(),
        );
    }

    None
}
