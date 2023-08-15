extern crate bcrypt;

use crate::data;
use crate::CompanyInfo;
use crate::JWT_KEY;
use bcrypt::{hash, verify, DEFAULT_COST};
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
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
        // println!("==> got back -> {:?}", company);
        // compare hashes and if ok then sign the token and send back
        let hashed = match hash(payload.password, DEFAULT_COST) {
            Ok(hash) => hash,
            Err(_) => "".to_string(),
        };

        if let Ok(is_valid) = verify(company.password, &hashed) {
            if let Ok(token) = encode(
                &Header::default(),
                &CompanyInfo {
                    id: company.id,
                    name: company.name,
                },
                &EncodingKey::from_secret(JWT_KEY.as_ref()),
            ) {
                return Ok(with_status(
                    json(&TokenResponse { token: token }),
                    StatusCode::OK,
                ));
            }
        }
    }
    Ok(with_status(json(&"Unauthorised"), StatusCode::UNAUTHORIZED))
}

pub async fn test_auth_handle(token: CompanyInfo) -> Result<impl warp::Reply, warp::Rejection> {
    println!("==> got token {:?}", token);

    Ok(with_status(
        json(&"todo"),
        StatusCode::INTERNAL_SERVER_ERROR,
    ))
}
