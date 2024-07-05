use lazy_static::lazy_static;
use snowflaked::Generator;
use std::sync::Mutex;
use tracing::error;

lazy_static! {
    static ref SNOWFLAKE_ID: Mutex<Generator> = Mutex::new(Generator::new(0));
}

pub fn generate_snowflake_id() -> u64 {
    SNOWFLAKE_ID
        .lock()
        .unwrap_or_else(|err| {
            error!("get snowflake generator err: {}", err);
            panic!("get snowflake generator err");
        })
        .generate::<u64>()
}
