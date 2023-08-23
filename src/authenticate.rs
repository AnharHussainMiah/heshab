use crate::data;
use crate::CompanyInfo;
use crate::JWT_KEY;
use jsonwebtoken::{encode, EncodingKey, Header};
use pbkdf2::{
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Pbkdf2,
};
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

        if Pbkdf2
            .verify_password(&payload.password.into_bytes(), &hashed.unwrap())
            .is_ok()
        {
            if let Ok(token) = encode(
                &Header::default(),
                &CompanyInfo {
                    id: company.id,
                    name: company.name,
                    exp: 10000000000,
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

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ResetPayload {
    email: String,
    password: String,
    token: String,
}

pub async fn reset_request(
    payload: ResetPayload,
    pool: PgPool,
) -> Result<impl warp::Reply, warp::Rejection> {
    let _ = data::request_password_reset(&pool, &payload.email).await;
    Ok(with_status(json(&"Success"), StatusCode::OK))
}

pub async fn update_new_password(
    payload: ResetPayload,
    pool: PgPool,
) -> Result<impl warp::Reply, warp::Rejection> {
    if payload.token.len() != 36 {
        return Ok(with_status(
            json(&"Token is invalid"),
            StatusCode::BAD_REQUEST,
        ));
    }

    if payload.email.is_empty() {
        return Ok(with_status(
            json(&"Email can not be empty"),
            StatusCode::BAD_REQUEST,
        ));
    }

    if payload.password.len() < 10 {
        return Ok(with_status(
            json(&"Password can not be less then 10 characters"),
            StatusCode::BAD_REQUEST,
        ));
    }

    if let Ok(is_valid_token) =
        data::is_valid_reset_token(&pool, &payload.token, &payload.email).await
    {
        if is_valid_token {
            let salt = SaltString::generate(&mut rand_core::OsRng);
            if let Ok(hash) = Pbkdf2.hash_password(payload.password.as_bytes(), &salt) {
                let encoded = format!("{}", hash);
                let _ = data::set_new_password(&pool, &payload.token, &encoded).await;
                return Ok(with_status(json(&"Success"), StatusCode::OK));
            }
        }
    }

    Ok(with_status(
        json(&"Either the email is invalid or the reset token has expired or is invalid"),
        StatusCode::BAD_REQUEST,
    ))
}
