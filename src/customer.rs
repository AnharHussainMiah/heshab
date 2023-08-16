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
) -> Result<impl warp::Reply, warp::Rejection> {
    println!("==> got company {:?}", company);
    Ok(with_status(
        json(&"todo"),
        StatusCode::INTERNAL_SERVER_ERROR,
    ))
}

pub async fn delete_customer_transaction(
    company: CompanyInfo,
) -> Result<impl warp::Reply, warp::Rejection> {
    println!("==> got company {:?}", company);
    Ok(with_status(
        json(&"todo"),
        StatusCode::INTERNAL_SERVER_ERROR,
    ))
}

pub async fn add_new_customer(company: CompanyInfo) -> Result<impl warp::Reply, warp::Rejection> {
    println!("==> got company {:?}", company);
    Ok(with_status(
        json(&"todo"),
        StatusCode::INTERNAL_SERVER_ERROR,
    ))
}

pub async fn update_customer(company: CompanyInfo) -> Result<impl warp::Reply, warp::Rejection> {
    println!("==> got company {:?}", company);
    Ok(with_status(
        json(&"todo"),
        StatusCode::INTERNAL_SERVER_ERROR,
    ))
}
