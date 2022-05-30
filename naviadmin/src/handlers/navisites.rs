use crate::state::AppState;
use crate::dbaccess::navisites::*;
use crate::models::navisites::{CreateNaviSite, UpdateNaviSite};
use crate::error::NaviError;
use std::convert::TryInto;
use actix_web::{web, HttpResponse, App, Responder};
use actix_multipart::Multipart;
use futures_util::TryStreamExt;
use uuid::Uuid;
use log::info;
use std::io::Write;
use futures_util::TryStreamExt as _;
use actix_web::web::Bytes;
use std::{str, fs};
use actix_web::http::StatusCode;
use crate::tools::common::get_image_content_type;

fn bytes_to_str(b: &Bytes) -> Result<&str, str::Utf8Error> {
    str::from_utf8(b)
}

pub async fn get_all_sites_handler(
    app_state: web::Data<AppState>) -> Result<HttpResponse, NaviError> {
    info!(" get all groups request");
    get_all_sites_db(&app_state.db)
        .await
        .map(|navi_sites| HttpResponse::Ok().json(navi_sites))
}

pub async fn create_new_site_handler(
    app_state: web::Data<AppState>,
    mut new_site: Multipart
) -> Result<HttpResponse, NaviError> {
    // init value
    let mut image_path: String = "images/default.png".to_string();
    let mut site_name: String = "".to_string();
    let mut site_description: String = "".to_string();
    let mut site_url: String = "".to_string();
    let mut site_group_id: i32 = 0;
    let mut update_id: i32 = 0;

    while let Some(mut field) = new_site.try_next().await? {
        let content_disposition = field.content_disposition();
        let image_content_dispostion = content_disposition.clone();
        let filename = content_disposition
            .get_name()
            .map_or_else(|| Uuid::new_v4().to_string(), sanitize_filename::sanitize);

        while let Some(chunk) = field.try_next().await? {
            if filename == "file" {
                let imagefilename = image_content_dispostion
                    .get_filename()
                    .map_or_else(|| Uuid::new_v4().to_string(), sanitize_filename::sanitize);
                let filepath = format!("images/{}", imagefilename);
                image_path = filepath.clone().to_string();
                let mut f = web::block(|| std::fs::File::create(filepath)).await??;
                web::block(move || f.write_all(&chunk).map(|_| f)).await??;
            } else if filename == "name" {
                let name_data = chunk;
                site_name = bytes_to_str(&name_data).unwrap().to_string();
            } else if filename == "site_url" {
                let site_url_data = chunk;
                site_url = bytes_to_str(&site_url_data).unwrap().to_string();
            } else if filename == "site_description" {
                let description_data = chunk;
                site_description = bytes_to_str(&description_data).unwrap().to_string();
            } else if filename == "site_group_id" {
                let group_id_data = chunk;
                site_group_id = bytes_to_str(&group_id_data).unwrap().to_string().parse::<i32>().unwrap();
            } else if filename == "update_id" {
                let update_id_data = chunk;
                update_id = bytes_to_str(&update_id_data).unwrap().to_string().parse::<i32>().unwrap();
            }
        }
    }
    let new_site_data = CreateNaviSite {
        name: Some(site_name),
        site_url: Some(site_url),
        site_description: Some(site_description),
        image_path: Some(image_path),
        site_group_id: site_group_id,
    };
     create_new_site_db(&app_state.db, new_site_data, update_id)
        .await
        .map(|navi_sites| HttpResponse::Ok().json(navi_sites))
}

pub async fn site_image_handler(
    params: web::Path<String>
) -> Result<HttpResponse, NaviError>{
    let image_name = params.into_inner();
    let content_type = get_image_content_type(&image_name);
    let image_path = format!("images/{}", image_name);
    let image_content = web::block(|| std::fs::read(image_path)).await?.unwrap();

    Ok(HttpResponse::build(StatusCode::OK)
        .content_type(content_type)
        .body(image_content))
}