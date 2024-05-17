use salvo::oapi::{ToParameters, ToSchema};
use serde::{Deserialize, Serialize};

// 定义一个结构体，用于接收查询参数
#[derive(Serialize, Deserialize, Debug, Default, ToParameters)]
pub struct QueryParamsStruct {
    #[salvo(parameter(
        parameter_in = Query, 
        required = false
    ))]
    pub language: Option<String>,
    #[salvo(parameter(
        parameter_in = Query, 
        required = false
    ))]
    pub category: Option<String>,
}

// 定义一个结构体，用于接收路径参数:资源的uuid
#[derive(ToParameters, Deserialize, Debug, Default)]
#[salvo(parameters(default_parameter_in = Path))]
pub struct PathFilterStruct{
    #[salvo(parameter(value_type = String))]
    pub resource: Option<String>,
}

// 定义一个结构体，用于接收请求体: 用于获取资源的上传账号名称
#[derive(Serialize, Deserialize, Debug, Default, ToSchema)]
#[salvo(parameters(default_parameter_in = Body))]
pub struct BodyFilterStruct{
    #[salvo(parameter(value_type = String))]
    pub auth_name: Option<String>,
}

// 定义一个结构体，用于接收请求体: 用于接收禁用/启用账号的uuid
#[derive(Serialize, Deserialize, Debug, Default, ToSchema)]
#[salvo(parameters(default_parameter_in = Body))]
pub struct BodyStructOfDE{
    #[salvo(parameter(value_type = String))]
    pub d_e_uuid: Option<String>,
}

// 定义一个结构体，用于接收请求体: 用于接收新建的语言
#[derive(Serialize, Deserialize, Debug, Default, ToSchema)]
#[salvo(parameters(default_parameter_in = Body))]
pub struct BodyStructCreateLanguage{
    #[salvo(parameter(value_type = String))]
    pub language: Option<String>,
}

// 定义一个结构体，用于接收请求体: 用于接收新建的分类
#[derive(Serialize, Deserialize, Debug, Default, ToSchema)]
#[salvo(parameters(default_parameter_in = Body))]
pub struct BodyStructCreateCategory{
    #[salvo(parameter(value_type = String))]
    pub category: Option<String>,
}

// 定义一个结构体，用于接收请求体: 用于接收删除的id
#[derive(Serialize, Deserialize, Debug, Default, ToSchema)]
#[salvo(parameters(default_parameter_in = Body))]
pub struct DeleteId{
    #[salvo(parameter(value_type = String))]
    pub c_l_id: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, Default, ToSchema)]
#[salvo(parameters(default_parameter_in = Body))]
pub struct DeleteUuid{
    #[salvo(parameter(value_type = String))]
    pub img_uuid: Option<String>,
}

// 定义一个结构体，用于接收分页参数
#[derive(Serialize, Deserialize, Debug, Default, ToSchema)]
#[salvo(parameters(default_parameter_in = Body))]
pub struct QueryPageStruct {
    pub page: Option<u64>,
    pub page_size: Option<u64>,
}
