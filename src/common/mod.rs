use salvo::{writing::Json, Request, Response};
use serde::{de::DeserializeOwned, Deserialize, Serialize};

use crate::cerror::CodeError;

pub mod response;

pub fn success<T>(response_param: &mut Response, data: T)
where
    T: Serialize + Send + Sync,
{
    response_param.render(Json(&response::Response::ok(data)));
}

pub fn failed(response_param: &mut Response, err: CodeError) {
    response_param.render(Json(&response::Response::<()>::err(err.0, err.1)));
}

pub async fn resolve_code_error<F, E, T>(response: &mut Response, callback: F)
where
    E: std::future::Future<Output = Result<T, CodeError>>,
    T: Serialize + Send + Sync,
    F: FnOnce() -> E,
{
    match callback().await {
        Ok(data) => {
            success(response, data);
        }
        Err(err) => {
            failed(response, err);
        }
    }
}

pub fn get_query_param<'a, T>(request: &'a mut Request) -> Option<T>
where
    T: DeserializeOwned,
{
    match request.parse_queries::<T>() {
        Ok(param) => Some(param),
        Err(_) => None,
    }
}
