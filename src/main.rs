mod authenticate;
mod logo;

use lazy_static::lazy_static;
use serde::de::DeserializeOwned;
use sqlx::PgPool;
use std::env;
use std::process;
use uuid::Uuid;
use warp::Filter;

const VERSION: &str = "0.1.0";

lazy_static! {
    static ref DBURL: String = self::load_key("DATABASE_URL");
}

#[tokio::main]
async fn main() {
    logo::draw(&VERSION);
    if let Ok(pool) = PgPool::connect(&DBURL).await {
        let _ = sqlx::migrate!().run(&pool).await;

        let public = warp::get().and(warp::fs::dir("public"));

        let post_auth = warp::post()
            .and(warp::path!("api" / "auth"))
            .and(self::extract_json_of::<authenticate::AuthenticatePayload>())
            .and(warp::any().map(move || pool.clone()))
            .and_then(authenticate::handle);

        let routes = public.or(post_auth);

        println!("==> serving application on port 0.0.0.0:8080 use CTL+C to stop..");
        warp::serve(routes).run(([0, 0, 0, 0], 8080)).await;
    } else {
        println!("WARNING: unable to establish a database connection, exiting...");
        process::exit(1);
    }
}

fn load_key(k: &str) -> String {
    return match env::var(k) {
        Ok(v) => v,
        Err(_) => Uuid::new_v4().to_string(),
    };
}

pub fn extract_json_of<T: DeserializeOwned + Send>(
) -> impl Filter<Extract = (T,), Error = warp::Rejection> + Clone {
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}
