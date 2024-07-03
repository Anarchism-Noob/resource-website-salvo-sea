use crate::{cerror::CodeError, dtos::casbin_resource_dto::{
    CreateCasbinResourceRequest, CreateCasbinResourceResponse, DeleteCasbinResourceRequest,
    DeleteCasbinResourceResponse, GetCasbinResourceRequest, GetCasbinResourceResponse,
    ListCasbinResourceRequest, ListCasbinResourceResponse, PageCasbinResourceRequest,
    PageCasbinResourceResponse, UpdateCasbinResourceRequest, UpdateCasbinResourceResponse,
}};

pub async fn list_casbin_resource(
    request: ListCasbinResourceRequest,
) -> anyhow::Result<ListCasbinResourceResponse, CodeError> {
    unimplemented!()
}

pub async fn page_casbin_resource(
    request: PageCasbinResourceRequest,
) -> anyhow::Result<PageCasbinResourceResponse, CodeError> {
    unimplemented!()
}

pub async fn get_casbin_resource(
    request: GetCasbinResourceRequest,
) -> anyhow::Result<GetCasbinResourceResponse, CodeError> {
    unimplemented!()
}

pub async fn create_casbin_resource(
    request: CreateCasbinResourceRequest,
) -> anyhow::Result<CreateCasbinResourceResponse, CodeError> {
    unimplemented!()
}

pub async fn update_casbin_resource(
    request: UpdateCasbinResourceRequest,
) -> anyhow::Result<UpdateCasbinResourceResponse, CodeError> {
    unimplemented!()
}

pub async fn delete_casbin_resource(
    request: DeleteCasbinResourceRequest,
) -> anyhow::Result<DeleteCasbinResourceResponse, CodeError> {
    unimplemented!()
}
