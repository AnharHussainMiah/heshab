mod authenticate;
mod customer;
mod data;
mod logo;
mod models;

use jsonwebtoken::{decode, DecodingKey, Validation};
use lazy_static::lazy_static;
use models::CompanyInfo;
use serde::de::DeserializeOwned;
use sqlx::PgPool;
use std::env;
use std::fs;
use std::process;
use uuid::Uuid;
use warp::http::StatusCode;
use warp::reject::Reject;
use warp::reply::{json, with_status};
use warp::Filter;
use warp::Rejection;
use warp::Reply;

const VERSION: &str = "0.1.0";

lazy_static! {
    static ref DBURL: String = self::load_key("DATABASE_URL");
    static ref JWT_KEY: String = self::load_key("JWT_KEY");
}

#[tokio::main]
async fn main() {
    logo::draw(VERSION);

    //let pass = "password".to_string();
    // if let Ok(hash) = hash(pass.into_bytes(), DEFAULT_COST) {
    //     println!("hash => {}", hash);
    // }

    // let salt = SaltString::generate(&mut rand_core::OsRng);
    // if let Ok(hash) = Pbkdf2.hash_password(pass.as_bytes(), &salt) {
    //     println!("hash -> {}", hash);
    // }

    if let Ok(pool) = PgPool::connect(&DBURL).await {
        let _ = sqlx::migrate!().run(&pool).await;

        let public = warp::get().and(warp::fs::dir("public"));

        let spa_root = warp::path::end().and(warp::any()).map(|| {
            match fs::read_to_string("public/index.html") {
                Ok(body) => warp::http::Response::builder()
                    .header("content-type", "text/html; charset=utf-8".to_string())
                    .status(200)
                    .body(body),
                Err(_) => warp::http::Response::builder()
                    .header("content-type", "text/html; charset=utf-8".to_string())
                    .status(404)
                    .body("not found".to_string()),
            }
        });

        // let spa_root_sub = warp::path::end().map(|q: i32|
        //     match fs::read_to_string("public/index.html") {
        //         Ok(body) => {
        //             warp::http::Response::builder()
        //                 .header("content-type", "text/html; charset=utf-8".to_string())
        //                 .status(200)
        //                 .body(body.to_string())
        //         },
        //         Err(_) => {
        //             warp::http::Response::builder()
        //                 .header("content-type", "text/html; charset=utf-8".to_string())
        //                 .status(404)
        //                 .body("not found".to_string())
        //         }
        //     }
        // );

        let get_spa_search = warp::get().and(warp::path("search")).and(spa_root);
        let get_spa_transactions = warp::get().and(warp::path("transactions")).and(spa_root);
        let get_spa_transactions_sub = warp::get()
            .and(warp::path!("transactions" / i32).map(|x: i32| x))
            .and_then(self::index);
        let get_spa_customer = warp::get().and(warp::path("customer")).and(spa_root);
        let get_spa_customer_sub = warp::get()
            .and(warp::path!("customer" / i32).map(|x: i32| x))
            .and_then(self::index);
        let get_spa_forgot_password = warp::get().and(warp::path("forgot-password")).and(spa_root);
        let get_spa_new_password = warp::get().and(warp::path("new-password")).and(spa_root);

        let p1 = pool.clone();
        let p2 = pool.clone();
        let p3 = pool.clone();
        let p4 = pool.clone();
        let p5 = pool.clone();
        let p6 = pool.clone();
        let p7 = pool.clone();
        let p8 = pool.clone();
        let p9 = pool.clone();
        let p10 = pool.clone();

        let post_auth = warp::post()
            .and(warp::path!("api" / "auth"))
            .and(self::extract_json_of::<authenticate::AuthenticatePayload>())
            .and(warp::any().map(move || p1.clone()))
            .and_then(authenticate::handle);

        let auth = warp::header("authorization")
            .map(|token: String| token)
            .and_then(self::validate_token);

        let post_search_customers = warp::post()
            .and(warp::path!("api" / "search-customers" / String).map(|query: String| query))
            .and(auth)
            .and(warp::any().map(move || p2.clone()))
            .and_then(customer::search_customers_by_name);

        let post_get_customer_detail = warp::post()
            .and(
                warp::path!("api" / "get-customer-detail" / i32)
                    .map(|customer_id: i32| customer_id),
            )
            .and(auth)
            .and(warp::any().map(move || p3.clone()))
            .and_then(customer::get_customer_detail);

        let post_get_customer_transactions = warp::post()
            .and(
                warp::path!("api" / "get-customer-transactions" / i32)
                    .map(|customer_id: i32| customer_id),
            )
            .and(auth)
            .and(warp::any().map(move || p4.clone()))
            .and_then(customer::get_customer_transactions);

        let post_add_transaction = warp::post()
            .and(warp::path!("api" / "add-customer-transaction"))
            .and(auth)
            .and(self::extract_json_of::<customer::TransactionPayload>())
            .and(warp::any().map(move || p5.clone()))
            .and_then(customer::add_customer_transaction);

        let post_delete_transaction = warp::post()
            .and(
                warp::path!("api" / "delete-customer-transaction" / i32)
                    .map(|transaction_id: i32| transaction_id),
            )
            .and(auth)
            .and(warp::any().map(move || p6.clone()))
            .and_then(customer::delete_customer_transaction);

        let post_add_new_customer = warp::post()
            .and(warp::path!("api" / "add-new-customer"))
            .and(auth)
            .and(self::extract_json_of::<customer::CustomerPayload>())
            .and(warp::any().map(move || p7.clone()))
            .and_then(customer::add_new_customer);

        let post_update_customer = warp::post()
            .and(warp::path!("api" / "update-customer"))
            .and(auth)
            .and(self::extract_json_of::<customer::CustomerPayload>())
            .and(warp::any().map(move || p8.clone()))
            .and_then(customer::update_new_customer);

        let post_confirm_is_logged_in = warp::post()
            .and(warp::path!("api" / "confirm-is-logged-in"))
            .and(auth)
            .and_then(self::confirm_logged_in);

        let post_reset_request = warp::post()
            .and(warp::path!("api" / "reset-request"))
            .and(self::extract_json_of::<authenticate::ResetPayload>())
            .and(warp::any().map(move || p9.clone()))
            .and_then(authenticate::reset_request);

        let post_update_new_password = warp::post()
            .and(warp::path!("api" / "update-new-password"))
            .and(self::extract_json_of::<authenticate::ResetPayload>())
            .and(warp::any().map(move || p10.clone()))
            .and_then(authenticate::update_new_password);

        let routes = public
            .or(get_spa_search)
            .or(get_spa_transactions)
            .or(get_spa_transactions_sub)
            .or(get_spa_customer)
            .or(get_spa_customer_sub)
            .or(get_spa_forgot_password)
            .or(get_spa_new_password)
            .or(post_auth)
            .or(post_search_customers)
            .or(post_get_customer_detail)
            .or(post_get_customer_transactions)
            .or(post_add_transaction)
            .or(post_delete_transaction)
            .or(post_add_new_customer)
            .or(post_update_customer)
            .or(post_confirm_is_logged_in)
            .or(post_reset_request)
            .or(post_update_new_password)
            .recover(self::handle_rejection);

        println!("==> serving application on port 0.0.0.0:8080 use CTL+C to stop..");
        warp::serve(routes).run(([0, 0, 0, 0], 8080)).await;
    } else {
        println!("WARNING: unable to establish a database connection, exiting...");
        process::exit(1);
    }
}

#[derive(Debug)]
struct InvalidToken;
impl Reject for InvalidToken {}

async fn validate_token(token: String) -> Result<CompanyInfo, Rejection> {
    let token_message = decode::<CompanyInfo>(
        &token,
        &DecodingKey::from_secret(JWT_KEY.as_ref()),
        &Validation::default(),
    );

    match token_message {
        Ok(data) => Ok(data.claims),
        Err(_) => Err(warp::reject::custom(InvalidToken)),
    }
}

async fn handle_rejection(err: Rejection) -> Result<impl Reply, std::convert::Infallible> {
    if err.is_not_found() {
        Ok(warp::reply::with_status("NOT_FOUND", StatusCode::NOT_FOUND))
    } else if let Some(e) = err.find::<InvalidToken>() {
        Ok(warp::reply::with_status(
            "Invalid Token",
            StatusCode::UNAUTHORIZED,
        ))
    } else {
        eprintln!("unhandled rejection: {:?}", err);
        Ok(warp::reply::with_status(
            "INTERNAL_SERVER_ERROR",
            StatusCode::INTERNAL_SERVER_ERROR,
        ))
    }
}

fn load_key(k: &str) -> String {
    match env::var(k) {
        Ok(v) => v,
        Err(_) => Uuid::new_v4().to_string(),
    }
}

pub fn extract_json_of<T: DeserializeOwned + Send>(
) -> impl Filter<Extract = (T,), Error = warp::Rejection> + Clone {
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

pub async fn confirm_logged_in(company: CompanyInfo) -> Result<impl warp::Reply, warp::Rejection> {
    Ok(with_status(json(&"success"), StatusCode::OK))
}

pub async fn index(_x: i32) -> Result<impl warp::Reply, warp::Rejection> {
    let f = match fs::read_to_string("public/index.html") {
        Ok(body) => warp::http::Response::builder()
            .header("content-type", "text/html; charset=utf-8".to_string())
            .status(200)
            .body(body),
        Err(_) => warp::http::Response::builder()
            .header("content-type", "text/html; charset=utf-8".to_string())
            .status(404)
            .body("not found".to_string()),
    };
    Ok(f)
}
