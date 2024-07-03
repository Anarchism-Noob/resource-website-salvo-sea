use tracing::field::debug;

use crate::{
    cerror::CodeError,
    dtos::system_user_dto::{
        CreateSystemUserRequest, CreateSystemUserResponse, DeleteSystemUserRequest,
        DeleteSystemUserResponse, GetSystemUserRequest, GetSystemUserResponse,
        ListSystemUserRequest, ListSystemUserResponse, PageSystemUserRequest,
        PageSystemUserResponse, SystemUser, UpdateSystemUserRequest, UpdateSystemUserResponse,
    },
};

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
    dbg!("get_system_user request: {:#?}", request);
    Ok(GetSystemUserResponse {
        data: SystemUser {
            id: "1".to_string(),
            name: "test".to_string(),
            nick_name: "test".to_string(),
            email: "test@test.com".to_string(),
            status: "active".to_string(),
            avatar_url: "https://test.com/avatar.png".to_string(),
        },
    })
}

pub async fn create_system_user(
    request: CreateSystemUserRequest,
) -> anyhow::Result<CreateSystemUserResponse, CodeError> {
    dbg!("create_system_user request: {:#?}", request);
    Ok(CreateSystemUserResponse {
        id: "1".to_string(),
    })
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
