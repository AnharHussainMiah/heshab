use crate::CompanyInfo;
use warp::http::StatusCode;
use warp::reply::{json, with_status};

pub async fn search_customers_by_name(
    company: CompanyInfo,
) -> Result<impl warp::Reply, warp::Rejection> {
    println!("==> got company {:?}", company);
    Ok(with_status(
        json(&"todo"),
        StatusCode::INTERNAL_SERVER_ERROR,
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
