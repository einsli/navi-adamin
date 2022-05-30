use crate::state::AppState;
use crate::dbaccess::navigroups::*;
use crate::models::navigroups::{CreateNaviGroup, UpdateNaviGroup};
use crate::error::NaviError;
use std::convert::TryInto;
use actix_web::{web, HttpResponse};
use log::info;

pub async fn get_all_groups_handler(
    app_state: web::Data<AppState>) -> Result<HttpResponse, NaviError> {
    info!(" get all groups request");
    get_all_groups_db(&app_state.db)
        .await
        .map(|navi_groups| HttpResponse::Ok().json(navi_groups))
}

pub async fn get_groups_detail_handler(
    app_state: web::Data<AppState>,
    params: web::Path<i32>
)-> Result<HttpResponse, NaviError> {
    let group_id = params.into_inner();
    get_group_details_db(&app_state.db, group_id)
        .await
        .map(|navi_group| HttpResponse::Ok().json(navi_group))
}

pub async fn delete_group_handler(
    app_state: web::Data<AppState>,
    params: web::Path<i32>
)-> Result<HttpResponse, NaviError> {
    info!("delete groups request");
    let group_id = params.into_inner();
    delete_group_db(&app_state.db, group_id)
        .await
        .map(|delete_group| HttpResponse::Ok().json(delete_group))
}

pub async fn save_group_handler(
    app_state: web::Data<AppState>,
    new_group: web::Json<CreateNaviGroup>
)-> Result<HttpResponse, NaviError> {
    info!("save groups request");
    save_group_db(&app_state.db, new_group.try_into()?)
        .await
        .map(|new_group| HttpResponse::Ok().json(new_group))
}

pub async fn update_group_handler(
    app_state: web::Data<AppState>,
    new_group: web::Json<UpdateNaviGroup>
) -> Result<HttpResponse, NaviError> {
    info!("update groups request");
    update_group_db(&app_state.db, new_group.try_into()?)
        .await
        .map(|new_group| HttpResponse::Ok().json(new_group))
}