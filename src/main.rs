mod logo;

use lazy_static::lazy_static;
use std::env;
use uuid::Uuid;

const VERSION: &str = "0.1.0";

lazy_static! {
    static ref DBURL: String = self::load_key("DATABASE_URL");
}

#[tokio::main]
async fn main() {
    logo::draw(&VERSION);
}

fn load_key(k: &str) -> String {
    return match env::var(k) {
        Ok(v) => v,
        Err(_) => Uuid::new_v4().to_string(),
    };
}