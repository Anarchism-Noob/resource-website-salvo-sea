use crate::{
    app_writer::AppResult,
    dtos::{
        sys_resources_dto::{
            SysResourceChangeLink, SysResourceCreateRequest, SysResourceList, SysResourceResponse,
        },
    },
    entities::{
        custom_user,
        prelude::{CustomOrders, SysImage, SysResourceImages, SysResources, SysUser},
        sys_image, sys_resource_images, sys_resources,
    },
    utils::{db::DB},
};
use chrono::{Local};
use sea_orm::{*};
use uuid::Uuid;

pub async fn delete_image(image_uuid: String, user_uuid: String) -> AppResult<()> {
    let db = DB.get().ok_or("数据库连接失败").unwrap();
    let user_query = SysUser::find_by_id(user_uuid.clone()).one(db).await?;
    if user_query.is_none() {
        return Err(anyhow::anyhow!("用户不存在").into());
    }
    // 根据前端传回的资源uuid查询资源信息
    let del_image = SysImage::delete_by_id(image_uuid.clone()).exec(db).await?;
    let query_middle = SysResourceImages::find()
        .filter(sys_resource_images::Column::ImageUuid.eq(image_uuid.clone()))
        .all(db)
        .await?;
    if query_middle.len() > 0 {
        let middle_id = query_middle[0].id.clone();
        let del_middle = SysResourceImages::delete_by_id(middle_id).exec(db).await?;
        return Ok(());
    }
    Ok(())
}

pub async fn get_resource_detail_by_uuid(
    resource_uuid: String,
    user_uuid: String,
    role: Option<u32>,
) -> AppResult<SysResourceResponse> {
    let db = DB.get().ok_or("数据库连接失败").unwrap();
    // 根据前端传回的资源uuid查询资源信息
    let resource_detail = SysResources::find_by_id(resource_uuid.clone())
        .one(db)
        .await?;
    // 获取资源的实体
    let resource_model = resource_detail.unwrap();
    // 根据当前用户的uuid查询订单信息
    let order_by_user = CustomOrders::find()
        .filter(custom_user::Column::UserUuid.eq(user_uuid.clone()))
        .filter(sys_resources::Column::ResourceUuid.eq(resource_uuid.clone()))
        .all(db)
        .await?;
    // 获取资源图片信息
    let resource_image_query = SysImage::find()
        .join_rev(
            JoinType::LeftJoin,
            sys_resource_images::Entity::belongs_to(sys_image::Entity)
                .from(sys_resource_images::Column::ImageUuid)
                .to(sys_image::Column::ImageUuid)
                .into(),
        )
        .filter(sys_resource_images::Column::ResourceUuid.eq(resource_uuid.clone()))
        .all(db)
        .await?;
    let description = if resource_model.description_file_path.is_none() {
        None
    } else {
        resource_model.description.clone()
    };
    let description_file_path = if resource_model.description_file_path.is_none() {
        None
    } else {
        resource_model.description_file_path.clone()
    };
    let mut srr_img = Vec::new();
    for item in resource_image_query.clone() {
        let resource_image_path = item.image_path.clone();
        srr_img.push(resource_image_path)
    }
    // 判断role是否为空
    if role.is_none() {
        // 判断用户是否购买资源
        if order_by_user.len() > 0 {
            let order_detail = order_by_user[0].clone();
            let resource_link = &order_detail.download_link.clone();

            // 构建 SysResourceResponse 结构体实例并返回
            return Ok(SysResourceResponse {
                resource_uuid: resource_model.resource_uuid,
                resource_name: resource_model.resource_name,
                description,
                description_file_path,
                resource_price: resource_model.resource_price,
                category: resource_model.category,
                language: resource_model.language,
                resource_link: resource_link.clone(),
                create_user_name: resource_model.create_user_name,
                resource_image: srr_img,
            });
        }
    } else {
        // 判断当前用户是否在管理员表中
        let admin_by_user = SysUser::find_by_id(user_uuid.clone()).one(db).await?;
        if admin_by_user.is_some() {
            // 返回完整的资源详情
            return Ok(SysResourceResponse {
                resource_uuid: resource_model.resource_uuid,
                resource_name: resource_model.resource_name,
                description,
                description_file_path,
                resource_price: resource_model.resource_price,
                category: resource_model.category,
                language: resource_model.language,
                resource_link: resource_model.resource_link,
                create_user_name: resource_model.create_user_name,
                resource_image: srr_img,
            });
        } else {
            return Err(anyhow::anyhow!("token被篡改，请重新登录").into());
        }
    }

    // 如果用户未购买资源，返回默认的 SysResourceResponse 实例
    Ok(SysResourceResponse {
        resource_uuid: resource_model.resource_uuid,
        resource_name: resource_model.resource_name,
        description,
        description_file_path,
        resource_price: resource_model.resource_price,
        category: resource_model.category,
        language: resource_model.language,
        resource_link: "".to_string(), // 这里可以填写默认的下载链接
        create_user_name: resource_model.create_user_name,
        resource_image: srr_img,
    })
}

pub async fn get_resources_by_category_and_language(
    category: String,
    language: String,
    page_no: u64,
    page_size: u64,
) -> AppResult<Vec<SysResourceList>> {
    let db = DB.get().ok_or("数据库连接失败").unwrap();
    let offset = (page_no - 1) * page_size;

    let resource_list = SysResources::find()
        .filter(sys_resources::Column::Category.eq(category))
        .filter(sys_resources::Column::Language.eq(language))
        .offset(offset)
        .limit(page_size)
        .all(db)
        .await?;
    let mut srr = Vec::new();
    for item in resource_list.clone() {
        let resource_uuid = &item.resource_uuid;
        let resource_image_query = SysImage::find()
            .join_rev(
                JoinType::LeftJoin,
                sys_resource_images::Entity::belongs_to(sys_image::Entity)
                    .from(sys_resource_images::Column::ImageUuid)
                    .to(sys_image::Column::ImageUuid)
                    .into(),
            )
            .filter(sys_resource_images::Column::ResourceUuid.eq(resource_uuid.clone()))
            .all(db)
            .await?;
        let resource_image_path = resource_image_query[0].image_path.clone();
        let res = SysResourceList {
            resource_uuid: item.resource_uuid.clone(),
            resource_name: item.resource_name.clone(),
            resource_price: item.resource_price.clone().into(),
            category: item.category.clone(),
            language: item.language.clone(),
            resource_image: resource_image_path,
        };
        srr.push(res)
    }
    Ok(srr)
}

pub async fn get_resours_of_language(
    language: String,
    page_no: u64,
    page_size: u64,
) -> AppResult<Vec<SysResourceList>> {
    let db = DB.get().ok_or("数据库连接失败").unwrap();
    let offset = (page_no - 1) * page_size;

    let resource_list = SysResources::find()
        .filter(sys_resources::Column::Language.eq(language))
        .offset(offset)
        .limit(page_size)
        .all(db)
        .await?;
    let mut srr = Vec::new();
    for item in resource_list.clone() {
        let resource_uuid = &item.resource_uuid;
        let resource_image_query = SysImage::find()
            .join_rev(
                JoinType::LeftJoin,
                sys_resource_images::Entity::belongs_to(sys_image::Entity)
                    .from(sys_resource_images::Column::ImageUuid)
                    .to(sys_image::Column::ImageUuid)
                    .into(),
            )
            .filter(sys_resource_images::Column::ResourceUuid.eq(resource_uuid.clone()))
            .all(db)
            .await?;
        let resource_image_path = resource_image_query[0].image_path.clone();
        let res = SysResourceList {
            resource_uuid: item.resource_uuid.clone(),
            resource_name: item.resource_name.clone(),
            resource_price: item.resource_price.clone().into(),
            category: item.category.clone(),
            language: item.language.clone(),
            resource_image: resource_image_path,
        };
        srr.push(res)
    }
    Ok(srr)
}

pub async fn get_resources_of_category(
    category: String,
    page_no: u64,
    page_size: u64,
) -> AppResult<Vec<SysResourceList>> {
    let db = DB.get().ok_or("数据库连接失败").unwrap();
    let offset = (page_no - 1) * page_size;

    let resource_list = SysResources::find()
        .filter(sys_resources::Column::Category.eq(category))
        .offset(offset)
        .limit(page_size)
        .all(db)
        .await?;
    let mut srr = Vec::new();
    for item in resource_list.clone() {
        let resource_uuid = &item.resource_uuid;
        let resource_image_query = SysImage::find()
            .join_rev(
                JoinType::LeftJoin,
                sys_resource_images::Entity::belongs_to(sys_image::Entity)
                    .from(sys_resource_images::Column::ImageUuid)
                    .to(sys_image::Column::ImageUuid)
                    .into(),
            )
            .filter(sys_resource_images::Column::ResourceUuid.eq(resource_uuid.clone()))
            .all(db)
            .await?;
        let resource_image_path = resource_image_query[0].image_path.clone();
        let res = SysResourceList {
            resource_uuid: item.resource_uuid.clone(),
            resource_name: item.resource_name.clone(),
            resource_price: item.resource_price.clone().into(),
            category: item.category.clone(),
            language: item.language.clone(),
            resource_image: resource_image_path,
        };
        srr.push(res)
    }
    Ok(srr)
}

pub async fn get_resource_list(page_no: u64, page_size: u64) -> AppResult<Vec<SysResourceList>> {
    let db = DB.get().ok_or("数据库连接失败").unwrap();
    let offset = (page_no - 1) * page_size;
    let resource_list = SysResources::find()
        .offset(offset)
        .limit(page_size)
        .all(db)
        .await?;
    let mut srr = Vec::new();
    for item in resource_list.clone() {
        let resource_uuid = &item.resource_uuid;
        let resource_image_query = SysImage::find()
            .join_rev(
                JoinType::LeftJoin,
                sys_resource_images::Entity::belongs_to(sys_image::Entity)
                    .from(sys_resource_images::Column::ImageUuid)
                    .to(sys_image::Column::ImageUuid)
                    .into(),
            )
            .filter(sys_resource_images::Column::ResourceUuid.eq(resource_uuid.clone()))
            .all(db)
            .await?;
        let resource_image_path = resource_image_query[0].image_path.clone();
        let res = SysResourceList {
            resource_uuid: item.resource_uuid.clone(),
            resource_name: item.resource_name.clone(),
            resource_price: item.resource_price.clone().into(),
            category: item.category.clone(),
            language: item.language.clone(),
            resource_image: resource_image_path,
        };
        srr.push(res)
    }
    Ok(srr)
}

pub async fn change_resource_link(form_data: SysResourceChangeLink) -> AppResult<()> {
    let db = DB.get().ok_or("数据库连接失败").unwrap();
    let model = sys_resources::Entity::find_by_id(form_data.resource_uuid.clone())
        .one(db)
        .await?;
    let mut model_res: sys_resources::ActiveModel = model.unwrap().into();
    model_res.resource_link = Set(form_data.resource_link.clone());
    model_res.update(db).await?;
    Ok(())
}

pub async fn create_resource(req: SysResourceCreateRequest, user_uuid: String) -> AppResult<()> {
    let db = DB.get().ok_or("数据库连接失败").unwrap();
    let user_model = SysUser::find_by_id(user_uuid).one(db).await?;
    if user_model.is_none() {
        return Err(anyhow::anyhow!("没有相关权限，请联系管理员").into());
    }

    let resource_active = sys_resources::ActiveModel {
        resource_uuid: Set(Uuid::new_v4().to_string()),
        resource_name: Set(req.resource_name),
        description: Set(req.description),
        description_file_path: Set(req.description_file_path),
        resource_price: Set(req.resource_price),
        category: Set(req.category),
        language: Set(req.language),
        resource_link: Set(req.resource_link),
        create_date: Set(Local::now().naive_utc()),
        create_user_name: Set(user_model.unwrap().user_name),
    }
    .save(db)
    .await?;

    let image_uuids = req.image_uuids;
    for item in image_uuids {
        let middle_resource_clone = resource_active.clone();
        let resource_name = middle_resource_clone.resource_name.unwrap().clone();
        let resource_uuid = middle_resource_clone.resource_uuid.unwrap().clone();
        // 创建资源图片关联表
        sys_resource_images::ActiveModel {
            id: Set(0),
            resource_uuid: Set(resource_uuid.clone()),
            image_uuid: Set(item.clone()),
        }
        .save(db)
        .await?;
        // 更新图片的描述和用途
        let image_active = SysImage::find_by_id(item.clone()).one(db).await?;
        let mut image_active_model: sys_image::ActiveModel = image_active.unwrap().into();
        image_active_model.description = Set(Some(format!("{}的展示图片", resource_name.clone())));
        image_active_model.usage_location = Set(Some("资源展示图片".to_string()));
        image_active_model.update(db).await?;
    }
    Ok(())
}

pub async fn save_resource_image(req: Vec<(String, String)>) -> AppResult<Vec<String>> {
    let db = DB.get().ok_or("数据库连接失败").unwrap();
    let mut image_uuids = Vec::new();
    for (path, file_name) in req {
        let active_model = sys_image::ActiveModel {
            image_uuid: Set(Uuid::new_v4().to_string()),
            image_name: Set(file_name.clone()),
            image_path: Set(path.clone()),
            ..Default::default()
        }
        .save(db)
        .await?;
        let save_uuid = active_model.image_uuid.unwrap();
        image_uuids.push(save_uuid.clone());
        println!("Saved image with UUID: {}", save_uuid);
    }
    Ok(image_uuids)
}
