mod logo;

use lazy_static::lazy_static;
use std::env;
use uuid::Uuid;
use warp::Filter;
use std::process;
use sqlx::PgPool;

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
        
        println!("==> serving application on port 0.0.0.0:8080 use CTL+C to stop..");
        warp::serve(public).run(([0, 0, 0, 0], 8080)).await;
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
