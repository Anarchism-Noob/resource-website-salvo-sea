use sea_orm::{EntityTrait, PaginatorTrait, QueryOrder, Set};
use tracing::error;

use crate::{
    cerror::{
        role::ERR_ROLE_NOT_FOUND, CodeError, ERR_DATABASE_CONNECT_FAILED,
        ERR_DATABASE_OPERATOR_FAILED,
    },
    dtos::system_role_dto::{
        CreateSystemRoleRequest, CreateSystemRoleResponse, DeleteSystemRoleRequest,
        DeleteSystemRoleResponse, GetSystemRoleRequest, GetSystemRoleResponse,
        ListSystemRoleRequest, ListSystemRoleResponse, PageSystemRoleRequest,
        PageSystemRoleResponse, SystemRoleDTO, UpdateSystemRoleRequest, UpdateSystemRoleResponse,
    },
    entities::system_role::{self, ActiveModel, Entity as SystemRole},
    utils::{db::DB, snowflake::generate_snowflake_id},
};

pub async fn list_system_role(
    _request: ListSystemRoleRequest,
) -> anyhow::Result<ListSystemRoleResponse, CodeError> {
    let conn = DB.get().ok_or(ERR_DATABASE_CONNECT_FAILED)?;

    let system_roles = SystemRole::find()
        .order_by_desc(system_role::Column::CreateTime)
        .all(conn)
        .await
        .map_err(|err| {
            error!("list system role err: {:#?}", err);
            ERR_DATABASE_OPERATOR_FAILED
        })?;

    Ok(ListSystemRoleResponse {
        data: system_roles
            .into_iter()
            .map(|system_role| role_to_role_dto(system_role))
            .collect(),
    })
}

pub async fn page_system_role(
    request: PageSystemRoleRequest,
) -> anyhow::Result<PageSystemRoleResponse, CodeError> {
    let conn = DB.get().ok_or(ERR_DATABASE_CONNECT_FAILED)?;

    let total = SystemRole::find().count(conn).await.map_err(|err| {
        error!("page system role count err: {:#?}", err);
        ERR_DATABASE_OPERATOR_FAILED
    })?;

    let system_roles = SystemRole::find()
        .order_by_desc(system_role::Column::CreateTime)
        .paginate(conn, request.page_size)
        .fetch_page(request.page_index)
        .await
        .map_err(|err| {
            error!("page system role err: {:#?}", err);
            ERR_DATABASE_OPERATOR_FAILED
        })?;

    Ok(PageSystemRoleResponse {
        data: system_roles
            .into_iter()
            .map(|system_role| role_to_role_dto(system_role))
            .collect(),
        total,
    })
}

pub async fn get_system_role(
    request: GetSystemRoleRequest,
) -> anyhow::Result<GetSystemRoleResponse, CodeError> {
    let conn = DB.get().ok_or(ERR_DATABASE_CONNECT_FAILED)?;

    let system_role = SystemRole::find_by_id(request.id)
        .one(conn)
        .await
        .map_err(|err| {
            error!("get system role err: {:#?}", err);
            ERR_DATABASE_OPERATOR_FAILED
        })?
        .ok_or_else(|| {
            error!("role: [{}] not found", &request.id);
            ERR_ROLE_NOT_FOUND
        })?;

    Ok(GetSystemRoleResponse {
        data: role_to_role_dto(system_role),
    })
}

pub async fn create_system_role(
    request: CreateSystemRoleRequest,
) -> anyhow::Result<CreateSystemRoleResponse, CodeError> {
    let conn = DB.get().ok_or(ERR_DATABASE_CONNECT_FAILED)?;

    let id = generate_snowflake_id();
    let system_role = system_role::ActiveModel {
        id: Set(id),
        name: Set(request.name),
        r#type: Set(request.r#type),
        desc: Set(request.desc),
        ..Default::default()
    };

    SystemRole::insert(system_role)
        .exec(conn)
        .await
        .map_err(|err| {
            error!("create system role err: {:#?}", err);
            ERR_DATABASE_OPERATOR_FAILED
        })?;

    Ok(CreateSystemRoleResponse { id })
}

pub async fn update_system_role(
    request: UpdateSystemRoleRequest,
) -> anyhow::Result<UpdateSystemRoleResponse, CodeError> {
    let conn = DB.get().ok_or(ERR_DATABASE_CONNECT_FAILED)?;

    let system_role = SystemRole::find_by_id(request.id)
        .one(conn)
        .await
        .map_err(|err| {
            error!("update system role err: {:#?}", err);
            ERR_DATABASE_OPERATOR_FAILED
        })?
        .ok_or_else(|| {
            error!("role: [{}] not found", &request.id);
            ERR_ROLE_NOT_FOUND
        })?;

    let mut system_role: ActiveModel = system_role.into();
    system_role.name = Set(request.name);
    system_role.r#type = Set(request.r#type);
    system_role.desc = Set(request.desc);

    SystemRole::update(system_role)
        .exec(conn)
        .await
        .map_err(|err| {
            error!("update system role err: {:#?}", err);
            ERR_DATABASE_OPERATOR_FAILED
        })?;

    Ok(UpdateSystemRoleResponse {})
}

pub async fn delete_system_role(
    request: DeleteSystemRoleRequest,
) -> anyhow::Result<DeleteSystemRoleResponse, CodeError> {
    let conn = DB.get().ok_or(ERR_DATABASE_CONNECT_FAILED)?;

    SystemRole::find_by_id(request.id)
        .one(conn)
        .await
        .map_err(|err| {
            error!("find system role err: {:#?}", err);
            ERR_DATABASE_OPERATOR_FAILED
        })?
        .ok_or_else(|| {
            error!("role: [{}] not found", &request.id);
            ERR_ROLE_NOT_FOUND
        })?;

    SystemRole::delete_by_id(request.id)
        .exec(conn)
        .await
        .map_err(|err| {
            error!("delete system role err: {:#?}", err);
            ERR_DATABASE_OPERATOR_FAILED
        })?;

    Ok(DeleteSystemRoleResponse {})
}

fn role_to_role_dto(role: system_role::Model) -> SystemRoleDTO {
    SystemRoleDTO {
        id: role.id,
        name: role.name,
        r#type: role.r#type,
        desc: role.desc,
    }
}
