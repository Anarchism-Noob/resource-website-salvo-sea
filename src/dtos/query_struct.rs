use salvo::oapi::ToParameters;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default, ToParameters)]
pub struct QueryParamsStruct {
    #[salvo(parameters(parameter_in = Path))]
    pub resource_uuid: Option<String>,
    #[salvo(parameters(parameter_in = Path))]
    pub user_uuid: Option<String>,
    #[salvo(parameters(parameter_in = Path))]
    pub id: Option<i32>,
    #[salvo(parameters(parameter_in = Query))]
    pub language: Option<String>,
    #[salvo(parameters(parameter_in = Query))]
    pub category: Option<String>,
    
    // #[salvo(extract(source(from = "body")))]
    // pub page: Option<u64>,
    // #[salvo(extract(source(from = "body")))]
    // pub page_size: Option<u64>,
}

#[derive(Serialize, Deserialize, Debug, Default, ToParameters)]
pub struct QueryPageStruct {
    pub page: Option<u64>,
    pub page_size: Option<u64>,
}
