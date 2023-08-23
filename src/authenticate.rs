use pbkdf2::{
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Pbkdf2,
};
use crate::data;
use crate::CompanyInfo;
use crate::JWT_KEY;
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
        // compare hashes and if ok then sign the token and send back
        let encoded_copy = company.password.clone();
        let hashed = PasswordHash::new(&encoded_copy);

        if Pbkdf2.verify_password(&payload.password.into_bytes(), &hashed.unwrap()).is_ok() {
            if let Ok(token) = encode(
                &Header::default(),
                &CompanyInfo {
                    id: company.id,
                    name: company.name,
                    exp: 10000000000
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
