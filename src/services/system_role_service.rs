use crate::{cerror::CodeError, dtos::system_role_dto::{
    CreateSystemRoleRequest, CreateSystemRoleResponse, DeleteSystemRoleRequest,
    DeleteSystemRoleResponse, GetSystemRoleRequest, GetSystemRoleResponse, ListSystemRoleRequest,
    ListSystemRoleResponse, PageSystemRoleRequest, PageSystemRoleResponse, UpdateSystemRoleRequest,
    UpdateSystemRoleResponse,
}};

pub async fn list_system_role(
    request: ListSystemRoleRequest,
) -> anyhow::Result<ListSystemRoleResponse, CodeError> {
    unimplemented!()
}

pub async fn page_system_role(
    request: PageSystemRoleRequest,
) -> anyhow::Result<PageSystemRoleResponse, CodeError> {
    unimplemented!()
}

pub async fn get_system_role(
    request: GetSystemRoleRequest,
) -> anyhow::Result<GetSystemRoleResponse, CodeError> {
    unimplemented!()
}

pub async fn create_system_role(
    request: CreateSystemRoleRequest,
) -> anyhow::Result<CreateSystemRoleResponse, CodeError> {
    unimplemented!()
}

pub async fn update_system_role(
    request: UpdateSystemRoleRequest,
) -> anyhow::Result<UpdateSystemRoleResponse, CodeError> {
    unimplemented!()
}

pub async fn delete_system_role(
    request: DeleteSystemRoleRequest,
) -> anyhow::Result<DeleteSystemRoleResponse, CodeError> {
    unimplemented!()
}
