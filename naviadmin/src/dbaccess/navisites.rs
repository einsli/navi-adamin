use crate::error::NaviError;
use crate::models::navisites::*;
use crate::response::NaviBaseResponse;
use mysql::Pool as mysqlPool;
use crate::tools::dbresponse::format_site_group;
use crate::tools::dbtools::{check_url_existed,create_sites_res, delete_db_res};
use log::info;
use futures_util::TryStreamExt as _;

pub async fn get_all_sites_db(pool: &mysqlPool
) -> Result<NaviBaseResponse<Vec<ListNaviSites>>, NaviError> {
    info!("processing get all sites ");
    let sites_groups: Vec<SearchSites> = pool.prep_exec(
        "SELECT np.id as group_id, np.name as group_name, ns.id as site_id, ns.name as site_name, ns.site_url, \
        ns.site_description, ns.site_img_path FROM navi_groups as np LEFT JOIN \
        navi_sites as ns ON ns.site_group_id = np.id WHERE np.is_deleted = 0 AND ns.is_deleted = 0;",
        ()
    ).map(
        |result| {
            result.map(|x| x.unwrap()).map(|row| {
                let (group_id, group_name, site_id, site_name, site_url, site_description, site_img_path
                ) = mysql::from_row(row);
                let sites = SearchSites {
                    group_id,
                    group_name,
                    site_id,
                    site_name,
                    site_url,
                    site_description,
                    site_img_path
                };
                sites
            }).collect()
        }
    ).unwrap();
    info!("processed get all sites done");
    match sites_groups.len() {
        0 => Err(NaviError::NotFound("Not site found".into())),
        _ => Ok(NaviBaseResponse{
            code: 0,
            message: "success".to_string(),
            data: format_site_group(&sites_groups)
        })
    }
}

pub async fn create_new_site_db(pool: &mysqlPool, new_site: CreateNaviSite,
                                update_id: i32) -> Result<NaviBaseResponse<String>, NaviError> {
    println!("update id {:?}", update_id);
    let site_name:String = new_site.name.unwrap();
    if site_name.clone() == "" {
        Err(NaviError::InvalidInput("name is invalid".into()))
    } else {
        if update_id == 0 {
            // new site added
            info!("new site creating!");
            if check_url_existed(pool, new_site.site_url.clone().unwrap()) {
                Err(NaviError::DBError("url is already existed!".to_string()))
            } else {
                let save_site_res = pool.prep_exec(
                    "INSERT INTO navi_sites(name, site_description, site_url, site_group_id, site_img_path) VALUES(?, ?, ?, ?, ?);",
                    (site_name, new_site.site_description, new_site.site_url, new_site.site_group_id,
                     new_site.image_path),

                );
                info!("processed create site done!");
                create_sites_res(save_site_res)
            }
        } else {
            // update site
            let save_site_res = pool.prep_exec(
                "UPDATE navi_sites SET name=?, site_description=?, site_url=?, site_group_id=?, site_img_path=? WHERE id = ?",
                (site_name, new_site.site_description, new_site.site_url, new_site.site_group_id, new_site.image_path, update_id)
            );
            info!("processed update site done!");
            create_sites_res(save_site_res)
        }
    }
}


pub async fn delete_site_db(pool: &mysqlPool, site_id: i32
) -> Result<NaviBaseResponse<String>, NaviError> {
    info!("processing delete site");
    let delete_res = pool.prep_exec(
        "UPDATE navi_sites SET is_deleted = 1 WHERE id = ?",
        (site_id,)
    );
    info!("processed delete done!");

    delete_db_res(delete_res)
}