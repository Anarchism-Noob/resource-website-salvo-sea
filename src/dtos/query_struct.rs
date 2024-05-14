use salvo::prelude::Extractible;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Extractible, Default)]
#[salvo(extract(default_source(from = "body")))]
pub struct QueryParamsStruct {
    #[salvo(extract(source(from = "param")))]
    pub resource_uuid: Option<String>,
    #[salvo(extract(source(from = "param")))]
    pub user_uuid: Option<String>,
    #[salvo(extract(source(from = "query")))]
    pub language: Option<String>,
    #[salvo(extract(source(from = "query")))]
    pub category: Option<String>,
    #[salvo(extract(source(from = "param")))]
    pub id: Option<i32>,
    #[salvo(extract(source(from = "body")))]
    pub page: Option<u64>,
    #[salvo(extract(source(from = "body")))]
    pub page_size: Option<u64>,
}
