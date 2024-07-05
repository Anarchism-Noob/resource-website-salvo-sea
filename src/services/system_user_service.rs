use casbin::RbacApi;
use tracing::error;

use sea_orm::{EntityTrait, PaginatorTrait, QueryOrder, Set};

use crate::{
    cerror::{
        rbac::ERR_RBAC_USER_ROLE_ADD_FAILED, user::ERR_USER_NOT_FOUND, CodeError,
        ERR_DATABASE_CONNECT_FAILED, ERR_DATABASE_OPERATOR_FAILED,
    },
    constant::USER_STATUS_ACTIVE,
    dtos::system_user_dto::{
        CreateSystemUserRequest, CreateSystemUserResponse, CurentUserRequest, CurentUserResponse,
        DeleteSystemUserRequest, DeleteSystemUserResponse, GetSystemUserRequest,
        GetSystemUserResponse, ListSystemUserRequest, ListSystemUserResponse,
        PageSystemUserRequest, PageSystemUserResponse, SystemUserDTO, UpdateSystemUserRequest,
        UpdateSystemUserResponse,
    },
    entities::system_user::{self, ActiveModel, Entity as SystemUser},
    utils::{casbin::CASBIN, db::DB, snowflake::generate_snowflake_id},
};

pub async fn list_system_user(
    _request: ListSystemUserRequest,
) -> anyhow::Result<ListSystemUserResponse, CodeError> {
    let conn = DB.get().ok_or(ERR_DATABASE_CONNECT_FAILED)?;

    let system_users = SystemUser::find()
        .order_by_desc(system_user::Column::CreateTime)
        .all(conn)
        .await
        .map_err(|err| {
            error!("list system user err: {:#?}", err);
            ERR_DATABASE_OPERATOR_FAILED
        })?;

    Ok(ListSystemUserResponse {
        data: system_users
            .into_iter()
            .map(|system_user| user_to_user_dto(system_user))
            .collect(),
    })
}

pub async fn page_system_user(
    request: PageSystemUserRequest,
) -> anyhow::Result<PageSystemUserResponse, CodeError> {
    let conn = DB.get().ok_or(ERR_DATABASE_CONNECT_FAILED)?;

    let total = SystemUser::find().count(conn).await.map_err(|err| {
        error!("page system user count err: {:#?}", err);
        ERR_DATABASE_OPERATOR_FAILED
    })?;

    let system_users = SystemUser::find()
        .order_by_desc(system_user::Column::CreateTime)
        .paginate(conn, request.page_size)
        .fetch_page(request.page_index)
        .await
        .map_err(|err| {
            error!("page system user err: {:#?}", err);
            ERR_DATABASE_OPERATOR_FAILED
        })?;

    Ok(PageSystemUserResponse {
        data: system_users
            .into_iter()
            .map(|system_user| user_to_user_dto(system_user))
            .collect(),
        total,
    })
}

pub async fn get_system_user(
    request: GetSystemUserRequest,
) -> anyhow::Result<GetSystemUserResponse, CodeError> {
    let conn = DB.get().ok_or(ERR_DATABASE_CONNECT_FAILED)?;

    let system_user = SystemUser::find_by_id(request.id)
        .one(conn)
        .await
        .map_err(|err| {
            error!("get system user err: {:#?}", err);
            ERR_DATABASE_OPERATOR_FAILED
        })?
        .ok_or_else(|| {
            error!("user: [{}] not found", &request.id);
            ERR_USER_NOT_FOUND
        })?;

    Ok(GetSystemUserResponse {
        data: user_to_user_dto(system_user),
    })
}

pub async fn create_system_user(
    request: CreateSystemUserRequest,
) -> anyhow::Result<CreateSystemUserResponse, CodeError> {
    let conn = DB.get().ok_or(ERR_DATABASE_CONNECT_FAILED)?;

    let id = generate_snowflake_id();
    let system_user = system_user::ActiveModel {
        id: Set(id),
        name: Set(request.name),
        nick_name: Set(request.nick_name),
        email: Set(request.email),
        status: Set(USER_STATUS_ACTIVE.to_owned()),
        avatar_url: Set(request.avatar_url),
        ..Default::default()
    };

    SystemUser::insert(system_user)
        .exec(conn)
        .await
        .map_err(|err| {
            error!("create system user err: {:#?}", err);
            ERR_DATABASE_OPERATOR_FAILED
        })?;

    if request.casbin_role_ids.len() > 0 {
        let mut guard = CASBIN.write().await;
        if let Some(ref mut enforcer) = *guard {
            enforcer
                .add_roles_for_user(
                    id.to_string().as_str(),
                    request
                        .casbin_role_ids
                        .iter()
                        .map(|resource_id| resource_id.to_string())
                        .collect::<Vec<String>>(),
                    None,
                )
                .await
                .map_err(|err| {
                    error!("add role for user err: {:#?}", err);
                    ERR_RBAC_USER_ROLE_ADD_FAILED
                })?;
        }
    }

    Ok(CreateSystemUserResponse { id })
}

pub async fn update_system_user(
    request: UpdateSystemUserRequest,
) -> anyhow::Result<UpdateSystemUserResponse, CodeError> {
    let conn = DB.get().ok_or(ERR_DATABASE_CONNECT_FAILED)?;

    let system_user = SystemUser::find_by_id(request.id)
        .one(conn)
        .await
        .map_err(|err| {
            error!("update system user err: {:#?}", err);
            ERR_DATABASE_OPERATOR_FAILED
        })?
        .ok_or_else(|| {
            error!("user: [{}] not found", &request.id);
            ERR_USER_NOT_FOUND
        })?;

    let mut system_user: ActiveModel = system_user.into();
    system_user.name = Set(request.name);
    system_user.nick_name = Set(request.nick_name);
    system_user.email = Set(request.email);
    system_user.status = Set(USER_STATUS_ACTIVE.to_owned());
    system_user.avatar_url = Set(request.avatar_url);

    SystemUser::update(system_user)
        .exec(conn)
        .await
        .map_err(|err| {
            error!("update system user err: {:#?}", err);
            ERR_DATABASE_OPERATOR_FAILED
        })?;

    Ok(UpdateSystemUserResponse {})
}

pub async fn delete_system_user(
    request: DeleteSystemUserRequest,
) -> anyhow::Result<DeleteSystemUserResponse, CodeError> {
    let conn = DB.get().ok_or(ERR_DATABASE_CONNECT_FAILED)?;

    SystemUser::find_by_id(request.id)
        .one(conn)
        .await
        .map_err(|err| {
            error!("find system user err: {:#?}", err);
            ERR_DATABASE_OPERATOR_FAILED
        })?
        .ok_or_else(|| {
            error!("user: [{}] not found", &request.id);
            ERR_USER_NOT_FOUND
        })?;

    SystemUser::delete_by_id(request.id)
        .exec(conn)
        .await
        .map_err(|err| {
            error!("delete system user err: {:#?}", err);
            ERR_DATABASE_OPERATOR_FAILED
        })?;

    Ok(DeleteSystemUserResponse {})
}

pub async fn current_user(
    request: CurentUserRequest,
) -> anyhow::Result<CurentUserResponse, CodeError> {
    let conn = DB.get().ok_or(ERR_DATABASE_CONNECT_FAILED)?;

    let system_user = SystemUser::find_by_id(request.id)
        .one(conn)
        .await
        .map_err(|err| {
            error!("find system user err: {:#?}", err);
            ERR_DATABASE_OPERATOR_FAILED
        })?
        .ok_or_else(|| {
            error!("user: [{}] not found", &request.id);
            ERR_USER_NOT_FOUND
        })?;

    let mut current_user_info = CurentUserResponse {
        data: user_to_user_dto(system_user),
        roles: vec![],
        resources: vec![],
    };

    let mut guard = CASBIN.write().await;
    if let Some(ref mut enforcer) = *guard {
        current_user_info.roles =
            enforcer.get_roles_for_user(request.id.to_string().as_str(), None);
        current_user_info.resources =
            enforcer.get_implicit_permissions_for_user(request.id.to_string().as_str(), None);
    }

    Ok(current_user_info)
}

fn user_to_user_dto(user: system_user::Model) -> SystemUserDTO {
    SystemUserDTO {
        id: user.id,
        name: user.name,
        nick_name: user.nick_name,
        email: user.email,
        status: user.status,
        avatar_url: user.avatar_url,
    }
}
