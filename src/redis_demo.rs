extern crate redis;
use redis::Commands;

pub async fn test_redis() -> redis::RedisResult<isize> {
    let client = redis::Client::open("redis://127.0.0.1/")?;
    let mut con = client.get_connection()?;
    let _ : () = con.set("my_key", 42)?;
    
    con.get("my_key")
}