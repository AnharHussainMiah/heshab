mod logo;

use lazy_static::lazy_static;
use std::env;
use uuid::Uuid;
use warp::Filter;

const VERSION: &str = "0.1.0";

lazy_static! {
    static ref DBURL: String = self::load_key("DATABASE_URL");
}

#[tokio::main]
async fn main() {
    logo::draw(&VERSION);
    let public = warp::get().and(warp::fs::dir("public"));

    println!("==> serving application on port 0.0.0.0:8080 use CTL+C to stop..");
    warp::serve(public).run(([0, 0, 0, 0], 8080)).await;
}

fn load_key(k: &str) -> String {
    return match env::var(k) {
        Ok(v) => v,
        Err(_) => Uuid::new_v4().to_string(),
    };
}
