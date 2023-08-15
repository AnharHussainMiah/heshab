extern crate bcrypt;

use bcrypt::{DEFAULT_COST, hash, verify};
use crate::data;
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

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TokenResponse {
    token: String,
}

pub async fn handle(
    payload: AuthenticatePayload,
    pool: PgPool,
) -> Result<impl warp::Reply, warp::Rejection> {
    if let Ok(company) = data::get_company_by_email(&pool, &payload.email).await {
        println!("==> got back -> {:?}", company);
        // compare hashes and if ok then sign the token and send back
        if let Ok(is_valid) = verify(company.password, &payload.password) {

        }
    }


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
