use crate::{cerror::CodeError, dtos::system_user_dto::{
    CreateSystemUserRequest, CreateSystemUserResponse, DeleteSystemUserRequest,
    DeleteSystemUserResponse, GetSystemUserRequest, GetSystemUserResponse, ListSystemUserRequest,
    ListSystemUserResponse, PageSystemUserRequest, PageSystemUserResponse, UpdateSystemUserRequest,
    UpdateSystemUserResponse,
}};

pub async fn list_system_user(
    request: ListSystemUserRequest,
) -> anyhow::Result<ListSystemUserResponse, CodeError> {
    unimplemented!()
}

pub async fn page_system_user(
    request: PageSystemUserRequest,
) -> anyhow::Result<PageSystemUserResponse, CodeError> {
    unimplemented!()
}

pub async fn get_system_user(
    request: GetSystemUserRequest,
) -> anyhow::Result<GetSystemUserResponse, CodeError> {
    unimplemented!()
}

pub async fn create_system_user(
    request: CreateSystemUserRequest,
) -> anyhow::Result<CreateSystemUserResponse, CodeError> {
    unimplemented!()
}

pub async fn update_system_user(
    request: UpdateSystemUserRequest,
) -> anyhow::Result<UpdateSystemUserResponse, CodeError> {
    unimplemented!()
}

pub async fn delete_system_user(
    request: DeleteSystemUserRequest,
) -> anyhow::Result<DeleteSystemUserResponse, CodeError> {
    unimplemented!()
}
