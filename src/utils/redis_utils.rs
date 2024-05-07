use bb8_redis::{
    bb8::Pool,
    redis, RedisConnectionManager,
};
pub use redis::AsyncCommands;
use tokio::sync::OnceCell;

use crate::config::CFG;

pub static REDIS_POOL: OnceCell<Pool<RedisConnectionManager>> = OnceCell::const_new();

pub async fn get_redis_pool() -> Pool<RedisConnectionManager> {
    println!("获取Redis连接中");

    let pool = REDIS_POOL.get_or_init(|| async {
        println!("Redis连接池初始化");
        let redis_url = CFG.cache.redis_url.as_str();
        let manager = RedisConnectionManager::new(redis_url).unwrap();
        Pool::builder().build(manager).await.unwrap()
    }).await;

    println!("获取Redis连接池成功");
    pool.clone()
}

pub async fn get_redis_connection() -> Pool<RedisConnectionManager> {
    let pool = get_redis_pool().await;
    pool
}
