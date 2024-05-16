use salvo::oapi::{ToParameters, ToSchema};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default, ToParameters)]
pub struct QueryParamsStruct {
    #[salvo(parameters(
        parameter_in = Path, 
        required = false
    ))]
    pub resource_uuid: Option<String>,
    // #[salvo(parameters(
    //     parameter_in = Path, 
    //     require = false
    // ))]
    // pub user_uuid: Option<String>,
    // #[salvo(parameters(
    //     parameter_in = Path, 
    //     require = false
    // ))]
    // pub id: Option<i32>,
    #[salvo(parameters(
        parameter_in = Query, 
        required = false
    ))]
    pub language: Option<String>,
    #[salvo(parameters(
        parameter_in = Query, 
        required = false
    ))]
    pub category: Option<String>,
    // #[salvo(extract(source(from = "body")))]
    // pub page: Option<u64>,
    // #[salvo(extract(source(from = "body")))]
    // pub page_size: Option<u64>,
}

#[derive(Serialize, Deserialize, Debug, Default, ToSchema)]
pub struct QueryPageStruct {
    pub page: Option<u64>,
    pub page_size: Option<u64>,
}
