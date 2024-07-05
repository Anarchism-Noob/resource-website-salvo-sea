use sea_orm::{EntityTrait, PaginatorTrait, QueryOrder, Set};
use tracing::error;

use crate::{
    cerror::{
        rbac::ERR_RBAC_RESOURCE_NOT_FOUND, CodeError, ERR_DATABASE_CONNECT_FAILED,
        ERR_DATABASE_OPERATOR_FAILED,
    },
    dtos::casbin_resource_dto::{
        CasbinResourceDTO, CreateCasbinResourceRequest, CreateCasbinResourceResponse,
        DeleteCasbinResourceRequest, DeleteCasbinResourceResponse, GetCasbinResourceRequest,
        GetCasbinResourceResponse, ListCasbinResourceRequest, ListCasbinResourceResponse,
        PageCasbinResourceRequest, PageCasbinResourceResponse, UpdateCasbinResourceRequest,
        UpdateCasbinResourceResponse,
    },
    entities::casbin_resource::{self, ActiveModel, Entity as CasbinResource},
    utils::{db::DB, snowflake::generate_snowflake_id},
};

pub async fn list_casbin_resource(
    _request: ListCasbinResourceRequest,
) -> anyhow::Result<ListCasbinResourceResponse, CodeError> {
    let conn = DB.get().ok_or(ERR_DATABASE_CONNECT_FAILED)?;

    let casbin_resources = CasbinResource::find()
        .order_by_desc(casbin_resource::Column::CreateTime)
        .all(conn)
        .await
        .map_err(|err| {
            error!("list casbin resource err: {:#?}", err);
            ERR_DATABASE_OPERATOR_FAILED
        })?;

    Ok(ListCasbinResourceResponse {
        data: casbin_resources
            .into_iter()
            .map(|casbin_resource| resource_to_resource_dto(casbin_resource))
            .collect(),
    })
}

pub async fn page_casbin_resource(
    request: PageCasbinResourceRequest,
) -> anyhow::Result<PageCasbinResourceResponse, CodeError> {
    let conn = DB.get().ok_or(ERR_DATABASE_CONNECT_FAILED)?;

    let total = CasbinResource::find().count(conn).await.map_err(|err| {
        error!("page casbin resource count err: {:#?}", err);
        ERR_DATABASE_OPERATOR_FAILED
    })?;

    let system_roles = CasbinResource::find()
        .order_by_desc(casbin_resource::Column::CreateTime)
        .paginate(conn, request.page_size)
        .fetch_page(request.page_index)
        .await
        .map_err(|err| {
            error!("page casbin resource err: {:#?}", err);
            ERR_DATABASE_OPERATOR_FAILED
        })?;

    Ok(PageCasbinResourceResponse {
        data: system_roles
            .into_iter()
            .map(|system_role| resource_to_resource_dto(system_role))
            .collect(),
        total,
    })
}

pub async fn get_casbin_resource(
    request: GetCasbinResourceRequest,
) -> anyhow::Result<GetCasbinResourceResponse, CodeError> {
    let conn = DB.get().ok_or(ERR_DATABASE_CONNECT_FAILED)?;

    let casbin_resource = CasbinResource::find_by_id(request.id)
        .one(conn)
        .await
        .map_err(|err| {
            error!("get casbin resource err: {:#?}", err);
            ERR_DATABASE_OPERATOR_FAILED
        })?
        .ok_or_else(|| {
            error!("casbin resource: [{}] not found", &request.id);
            ERR_DATABASE_OPERATOR_FAILED
        })?;

    Ok(GetCasbinResourceResponse {
        data: resource_to_resource_dto(casbin_resource),
    })
}

pub async fn create_casbin_resource(
    request: CreateCasbinResourceRequest,
) -> anyhow::Result<CreateCasbinResourceResponse, CodeError> {
    let conn = DB.get().ok_or(ERR_DATABASE_CONNECT_FAILED)?;

    let id = generate_snowflake_id();
    let casbin_resource = casbin_resource::ActiveModel {
        id: Set(id),
        name: Set(request.name),
        resource_type: Set(request.resource_type),
        display_name: Set(request.display_name),
        r#type: Set(request.r#type),
        resource_id: Set(request.resource_id),
        parent_id: Set(request.parent_id),
        ..Default::default()
    };

    CasbinResource::insert(casbin_resource)
        .exec(conn)
        .await
        .map_err(|err| {
            error!("create casbin resource err: {:#?}", err);
            ERR_DATABASE_OPERATOR_FAILED
        })?;

    Ok(CreateCasbinResourceResponse { id })
}

pub async fn update_casbin_resource(
    request: UpdateCasbinResourceRequest,
) -> anyhow::Result<UpdateCasbinResourceResponse, CodeError> {
    let conn = DB.get().ok_or(ERR_DATABASE_CONNECT_FAILED)?;

    let casbin_resource = CasbinResource::find_by_id(request.id)
        .one(conn)
        .await
        .map_err(|err| {
            error!("update casbin resource err: {:#?}", err);
            ERR_DATABASE_OPERATOR_FAILED
        })?
        .ok_or_else(|| {
            error!("casbin resource: [{}] not found", &request.id);
            ERR_RBAC_RESOURCE_NOT_FOUND
        })?;

    let mut casbin_resource: ActiveModel = casbin_resource.into();
    casbin_resource.name = Set(request.name);
    casbin_resource.resource_type = Set(request.resource_type);
    casbin_resource.display_name = Set(request.display_name);
    casbin_resource.r#type = Set(request.r#type);
    casbin_resource.resource_id = Set(request.resource_id);
    casbin_resource.parent_id = Set(request.parent_id);

    CasbinResource::update(casbin_resource)
        .exec(conn)
        .await
        .map_err(|err| {
            error!("update system role err: {:#?}", err);
            ERR_DATABASE_OPERATOR_FAILED
        })?;

    Ok(UpdateCasbinResourceResponse {})
}

pub async fn delete_casbin_resource(
    request: DeleteCasbinResourceRequest,
) -> anyhow::Result<DeleteCasbinResourceResponse, CodeError> {
    let conn = DB.get().ok_or(ERR_DATABASE_CONNECT_FAILED)?;

    CasbinResource::find_by_id(request.id)
        .one(conn)
        .await
        .map_err(|err| {
            error!("find casbin resource err: {:#?}", err);
            ERR_DATABASE_OPERATOR_FAILED
        })?
        .ok_or_else(|| {
            error!("casbin resource: [{}] not found", &request.id);
            ERR_RBAC_RESOURCE_NOT_FOUND
        })?;

    CasbinResource::delete_by_id(request.id)
        .exec(conn)
        .await
        .map_err(|err| {
            error!("delete casbin resource err: {:#?}", err);
            ERR_DATABASE_OPERATOR_FAILED
        })?;

    Ok(DeleteCasbinResourceResponse {})
}

fn resource_to_resource_dto(resource: casbin_resource::Model) -> CasbinResourceDTO {
    CasbinResourceDTO {
        id: resource.id,
        name: resource.name,
        resource_type: resource.resource_type,
        display_name: resource.display_name,
        r#type: resource.r#type,
        resource_id: resource.resource_id,
        parent_id: resource.parent_id,
    }
}
