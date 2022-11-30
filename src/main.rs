mod redis_demo;
mod sqlx_demo;

#[tokio::main]
async fn main() {
    sqlx_demo::test_postgres().await;
    redis_demo::test_redis().await;
}