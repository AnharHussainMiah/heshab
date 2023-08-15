use crate::CompanyInfo;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use warp::http::StatusCode;
use warp::reply::{json, with_status};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct AuthenticatePayload {
    email: String,
    password: String,
}

pub async fn handle(
    payload: AuthenticatePayload,
    pool: PgPool,
) -> Result<impl warp::Reply, warp::Rejection> {
    Ok(with_status(
        json(&"tood"),
        StatusCode::INTERNAL_SERVER_ERROR,
    ))
}

pub async fn test_auth_handle(token: CompanyInfo) -> Result<impl warp::Reply, warp::Rejection> {
    println!("==> got token {:?}", token);

    Ok(with_status(
        json(&"todo"),
        StatusCode::INTERNAL_SERVER_ERROR,
    ))
}
