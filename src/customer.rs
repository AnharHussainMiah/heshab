use crate::CompanyInfo;
use warp::http::StatusCode;
use warp::reply::{json, with_status};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Debug, Serialize, Deserialize)]
pub struct SingleStringPayload {
    pub data: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SingleIntPayload {
    pub data: i32
}

pub async fn search_customers_by_name(
    company: CompanyInfo,
    payload: SingleStringPayload,
    pool: PgPool 
) -> Result<impl warp::Reply, warp::Rejection> {
    
    if let Ok(customers) = crate::data::search_customers(&pool, &payload.data, &company.id).await {
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
    company: CompanyInfo,
) -> Result<impl warp::Reply, warp::Rejection> {
    println!("==> got company {:?}", company);
    Ok(with_status(
        json(&"todo"),
        StatusCode::INTERNAL_SERVER_ERROR,
    ))
}

pub async fn get_customer_transactions(
    company: CompanyInfo,
) -> Result<impl warp::Reply, warp::Rejection> {
    println!("==> got company {:?}", company);
    Ok(with_status(
        json(&"todo"),
        StatusCode::INTERNAL_SERVER_ERROR,
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
