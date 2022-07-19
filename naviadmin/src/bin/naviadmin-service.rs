use actix_web::{web, App, HttpServer};
use std::io;
use dotenv::dotenv;
use std::sync::Mutex;
use std::env;
use actix_cors::Cors;
use actix_web::http::header;
use crate::error::NaviError;
use mysql::Pool as mysqlPool;
use crate::routers::{general_routes, groups_routes, sites_routes};
use crate::state::AppState;
use crate::navilogs::init_navi_logs;

#[path = "../error.rs"]
mod error;
#[path = "../response.rs"]
mod response;
#[path = "../state.rs"]
mod state;
#[path = "../routers.rs"]
mod routers;
#[path = "../handlers/mod.rs"]
mod handlers;
#[path = "../dbaccess/mod.rs"]
mod dbaccess;
#[path = "../models/mod.rs"]
mod models;
#[path = "../tools/mod.rs"]
mod tools;
#[path = "../navilogs.rs"]
mod navilogs;

#[actix_web::main]
async fn main() -> io::Result<()>{
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect(
        "DATABASE_URL is not set.");

    let db_pool = mysqlPool::new(&database_url).unwrap();

    let shared_data = web::Data::new(AppState{
        health_check_response: "NaviAdmin".to_string(),
        visit_count: Mutex::new(0),
        db: db_pool
    });

    init_navi_logs();

    let app
        = move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT, header::ACCESS_CONTROL_ALLOW_ORIGIN])
            .allowed_header(header::CONTENT_TYPE)
            .allow_any_origin()
            .supports_credentials()
            .max_age(3600);
        App::new()
            .wrap(cors)
            .app_data(shared_data.clone())
            .app_data(web::JsonConfig::default().error_handler(|_err, _req| {
                NaviError::InvalidInput("invalid Json input".to_string()).into()
            }))
            .configure(general_routes)
            .configure(groups_routes)
            .configure(sites_routes)

    };
    HttpServer::new(app).bind("127.0.0.1:3000")?.run().await
}
