use crate::CompanyInfo;
use crate::models::Transactions;
use warp::http::StatusCode;
use warp::reply::{json, with_status};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;


#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionPayload {
    pub customer_id: i32,
    pub amount: i32
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CustomerPayload {
    pub customer_id: i32,
    pub name: String,
    pub phone: String,
    pub address: String
}

pub async fn search_customers_by_name(
    query: String,
    company: CompanyInfo,
    pool: PgPool 
) -> Result<impl warp::Reply, warp::Rejection> {
    
    if let Ok(customers) = crate::data::search_customers(&pool, &query, &company.id).await {
        return Ok(with_status(
                json(&customers),
                StatusCode::OK
            ));
    }

    Ok(with_status(
        json(&"No Data"),
        StatusCode::NOT_FOUND,
    ))
}

pub async fn get_customer_detail(
    customer_id: i32,
    company: CompanyInfo,
    pool: PgPool
) -> Result<impl warp::Reply, warp::Rejection> {

    if let Ok(customer) = crate::data::get_customer_detail(&pool, &customer_id, &company.id).await {
        return Ok(with_status(
            json(&customer),
            StatusCode::OK
        ));
    }

    Ok(with_status(
        json(&"No Data"),
        StatusCode::NOT_FOUND,
    ))
}

pub async fn get_customer_transactions(
    customer_id: i32,
    company: CompanyInfo,
    pool: PgPool
) -> Result<impl warp::Reply, warp::Rejection> {
    
    if let Ok(transactions) = crate::data::get_customer_transactions(&pool, &customer_id, &company.id).await {
        return Ok(with_status(
                json(&transactions),
                StatusCode::OK
            ));
    }

    Ok(with_status(
        json(&"No Data"),
        StatusCode::NOT_FOUND,
    ))
}

pub async fn add_customer_transaction(
    company: CompanyInfo,
    payload: TransactionPayload,
    pool: PgPool
) -> Result<impl warp::Reply, warp::Rejection> {
    
    if let Ok(_) = crate::data::add_customer_transaction(&pool, &payload.customer_id, &company.id, &payload.amount).await {
        return Ok(with_status(
            json(&"success".to_string()),
            StatusCode::OK
        ));
    }

    Ok(with_status(
        json(&"Unable to add customer transaction"),
        StatusCode::INTERNAL_SERVER_ERROR,
    ))
}

pub async fn delete_customer_transaction(
    transaction_id: i32,
    company: CompanyInfo,
    pool: PgPool
) -> Result<impl warp::Reply, warp::Rejection> {
    
    if let Ok(_) = crate::data::delete_customer_transaction(&pool, &company.id, &transaction_id).await {
        return Ok(with_status(
            json(&"success".to_string()),
            StatusCode::OK
        ));
    }

    Ok(with_status(
        json(&"Unable to delete customer transaction"),
        StatusCode::INTERNAL_SERVER_ERROR,
    ))
}

pub async fn add_new_customer(
    company: CompanyInfo,
    customer: CustomerPayload,
    pool: PgPool
) -> Result<impl warp::Reply, warp::Rejection> {
    
    if let Some(error) = self::validate_cusomter_error(&customer) {
        return Ok(with_status(
            json(&error),
            StatusCode::BAD_REQUEST
        ));
    }

    if let Ok(_) = crate::data::add_new_customer(&pool, &company.id, &customer).await {
        return Ok(with_status(
            json(&"success".to_string()),
            StatusCode::OK
        ));
    }

    Ok(with_status(
        json(&"Unable to delete customer transaction"),
        StatusCode::INTERNAL_SERVER_ERROR,
    ))
}

pub async fn update_new_customer(
    company: CompanyInfo,
    customer: CustomerPayload,
    pool: PgPool
) -> Result<impl warp::Reply, warp::Rejection> {
    
    if let Some(error) = self::validate_cusomter_error(&customer) {
        return Ok(with_status(
            json(&error),
            StatusCode::BAD_REQUEST
        ));
    }

    if let Ok(_) = crate::data::add_new_customer(&pool, &company.id, &customer).await {
        return Ok(with_status(
            json(&"success".to_string()),
            StatusCode::OK
        ));
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
    
    if customer.name.len() > 100 {
        return Some("Sorry the customer's name can not be longer then 100 characters".to_string());
    }

    if customer.phone.len() > 11 {
        return Some("Sorry the customer's phone only have 11 digits".to_string());
    }

    if customer.address.len() > 200 {
        return Some("Sorry the customer's addres can not be longer then 200 characters".to_string());
    }

    None
}
