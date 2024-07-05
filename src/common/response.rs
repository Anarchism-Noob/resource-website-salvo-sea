use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Response<T> {
    pub success: bool,
    pub code: i32,
    pub message: &'static str,
    pub data: Option<T>,
}

impl<T> Response<T> {
    pub fn ok(data: T) -> Self {
        Response {
            success: true,
            code: 0,
            message: "",
            data: Some(data),
        }
    }

    pub fn err(code: i32, message: &'static str) -> Self {
        Response {
            success: false,
            code,
            message,
            data: None,
        }
    }
}
