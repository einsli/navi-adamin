use crate::handlers::{general::*, navigroups::*};
use actix_web::web;
use actix_web::web::ServiceConfig;
use crate::handlers::navisites::{get_all_sites_handler, create_new_site_handler,site_image_handler};

pub fn general_routes(cfg: &mut ServiceConfig) {
    cfg.route("/health", web::get().to(health_check_handler));
}

pub fn groups_routes(cfg: &mut ServiceConfig) {
    cfg
        .service(web::scope("/groups")
            .route("/", web::get().to(get_all_groups_handler))
            .route("/", web::post().to(save_group_handler))
            .route("/{group_id}", web::get().to(get_groups_detail_handler))
            .route("/delete/{group_id}",web::post().to(delete_group_handler))
            .route("/update/", web::post().to(update_group_handler))
        );
}

pub fn sites_routes(cfg: &mut ServiceConfig) {
    cfg
        .service(web::scope("/sites")
            .route("/", web::get().to(get_all_sites_handler))
            .route("/new/", web::post().to(create_new_site_handler))
            .route("/images/{image_name}", web::get().to(site_image_handler))
        );
}