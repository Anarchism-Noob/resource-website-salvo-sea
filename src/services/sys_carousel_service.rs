use crate::{
    app_writer::AppResult,
    dtos::sys_carousel_dto::{CreateCarouselRequest, QueryCarouselResponse},
    entities::{
        prelude::{SysCarousel, SysImage, SysUser},
        sys_carousel, sys_image,
    },
    utils::db::DB,
};
use sea_orm::*;
use uuid::Uuid;

pub async fn create_carouwsel(form_data: CreateCarouselRequest, uuid: String) -> AppResult<()> {
    let db = DB.get().ok_or(anyhow::anyhow!("数据库连接失败")).unwrap();
    let admin_query = SysUser::find_by_id(&uuid).one(db).await?;
    let _admin_model = admin_query.ok_or(anyhow::anyhow!("管理员不存在"));

    // 更新图片信息
    let image_query = SysImage::find_by_id(&form_data.image_uuid).one(db).await?;
    let mut image_res: sys_image::ActiveModel = image_query.unwrap().into();
    image_res.image_to = Set(form_data.image_to_url);
    image_res.description = Set(form_data.image_to_description);
    image_res.update(db).await?;

    // 创建轮播图
    sys_carousel::ActiveModel {
        image_uuid: Set(form_data.image_uuid),
        ..Default::default()
    }
    .save(db)
    .await?;
    Ok(())
}

#[allow(dead_code)]
pub async fn save_carsousel(image_path: String, _file_name: String) -> AppResult<String> {
    let db = DB.get().ok_or(anyhow::anyhow!("数据库连接失败")).unwrap();
    let res = sys_image::ActiveModel {
        image_uuid: Set(Uuid::new_v4().to_string()),
        image_name: Set(_file_name),
        image_path: Set(image_path),
        ..Default::default()
    }
    .save(db)
    .await?;

    let image_uuid = res.image_uuid.unwrap();
    Ok(image_uuid)
}

pub async fn get_carousel() -> AppResult<Vec<QueryCarouselResponse>> {
    let db = DB.get().ok_or(anyhow::anyhow!("数据库连接失败")).unwrap();
    let res = sys_carousel::Entity::find().all(db).await?;

    let mut res_vec: Vec<QueryCarouselResponse> = Vec::new();
    for item in res {
        let image_uuid = item.image_uuid;
        let image_query = SysImage::find_by_id(&image_uuid).one(db).await?;
        let image_model = image_query.unwrap();

        let res_item = QueryCarouselResponse {
            id: item.id,
            image_uuid: image_model.image_uuid,
            carousel_url: image_model.image_path,
            image_to_url: image_model.image_to,
        };
        res_vec.push(res_item);
    }
    Ok(res_vec)
}

#[allow(dead_code)]
pub async fn delete_carousel(image_uuid: String, admin_uuid: String) -> AppResult<()> {
    let db = DB.get().ok_or(anyhow::anyhow!("数据库连接失败")).unwrap();
    let admin_query = SysUser::find_by_id(&admin_uuid).one(db).await?;
    let _admin_model = admin_query.ok_or(anyhow::anyhow!("管理员不存在"));
    match _admin_model {
        Ok(_) => {}
        Err(_) => {
            return Err(anyhow::anyhow!("管理员不存在").into());
        }
    }

    let carousel_query = SysCarousel::find()
        .filter(sys_carousel::Column::ImageUuid.eq(&image_uuid))
        .one(db)
        .await?;
    let carousel_id = carousel_query.unwrap().id;
    SysCarousel::delete_by_id(carousel_id).exec(db).await?;
    SysImage::delete_by_id(&image_uuid).exec(db).await?;
    Ok(())
}
