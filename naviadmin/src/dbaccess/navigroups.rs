use crate::error::NaviError;
use crate::models::navigroups::{CreateNaviGroup, UpdateNaviGroup, NaviGroup, ListNaviGroup};
use crate::response::NaviBaseResponse;
use crate::tools::dbtools::delete_db_res;
use mysql::Pool as mysqlPool;
use log::info;

pub async fn get_all_groups_db(pool: &mysqlPool
) -> Result<NaviBaseResponse<Vec<ListNaviGroup>>, NaviError> {
    info!("processing get all groups");
    let navigroups: Vec<ListNaviGroup> = pool.prep_exec(
        "SELECT id, name FROM navi_groups WHERE is_deleted=0",
        ()
    ).map(
        |result| {
            result.map(|x| x.unwrap()).map(|row| {
                let (id, name) = mysql::from_row(row);
                ListNaviGroup{
                    id,
                    name,
                }
            }).collect()
        }
    ).unwrap();
    info!("processed get all groups done!");
    match navigroups.len() {
        0 => Err(NaviError::NotFound("Not groups found".into())),
        _ => Ok(
            NaviBaseResponse{
                code: 0,
                message: "success".to_string(),
                data: navigroups
            })
    }
}

pub async fn get_group_details_db(pool: &mysqlPool, group_id: i32
) -> Result<NaviBaseResponse<NaviGroup>, NaviError> {
    info!("processing get group details");
    let mut navigroups: Vec<NaviGroup> = pool.prep_exec(
        "SELECT * FROM navi_groups WHERE is_deleted = 0 and id = ?",
        (group_id,)
    ).map(|result| {
        result
            .map(|x| x.unwrap())
            .map(|row| {
                let (id, name, is_deleted) = mysql::from_row(row);
                NaviGroup {
                    id,
                    name,
                    is_deleted,
                }
            })
            .collect()
    })
        .unwrap();
    info!("get groups done!");
    match navigroups.len() {
        0 => Err(NaviError::NotFound("Not groups found".into())),
        _ => Ok(
            NaviBaseResponse {
                code: 0,
                message: "success".to_string(),
                data: navigroups.pop().unwrap()})
    }
}

pub async fn delete_group_db(pool: &mysqlPool, group_id: i32
) -> Result<NaviBaseResponse<String>, NaviError> {
    info!("processing delete group");
    let delete_res = pool.prep_exec(
        "UPDATE navi_groups SET is_deleted = 1 WHERE id = ?",
        (group_id,)
    );
    info!("processed delete done!");
    delete_db_res(delete_res)
}

pub async fn save_group_db(pool: &mysqlPool, new_group: CreateNaviGroup
) -> Result<NaviBaseResponse<String>, NaviError> {
    info!("processing save group");
    let group_name:String = new_group.name.unwrap();
    if group_name.clone() == "" {
        Err(NaviError::InvalidInput("name is invalid".into()))
    } else {
        let save_res  = pool.prep_exec(
            "INSERT INTO navi_groups(name) VALUES(?)",
            (group_name,)
        );
        info!("processed save group done!");
        match save_res {
            Ok(res) => if res.affected_rows() == 0 {
                Err(NaviError::NotFound("No groups saved!".to_string()))
            } else {
                Ok(NaviBaseResponse{
                    code: 0,
                    message: "success".to_string(),
                    data: "".to_string(),
                })
            }
            Err(err) => Err(NaviError::DBError(err.to_string()))
        }
    }
}

pub async fn update_group_db(pool: &mysqlPool, new_group: UpdateNaviGroup
) -> Result<NaviBaseResponse<String>, NaviError> {
    info!("processing update group");
    let group_name:String = new_group.name.unwrap();
    if group_name.clone() == "" {
        Err(NaviError::InvalidInput("name is invalid".into()))
    } else {
        let update_res = pool.prep_exec(
            "UPDATE navi_groups SET name = ? WHERE id = ?",
            (group_name, new_group.id,)
        );

        info!("processed update group done!");
        match update_res {
            Ok(res) => if res.affected_rows() == 0 {
                Err(NaviError::NotFound("No groups found".to_string()))
            } else {
                Ok(NaviBaseResponse{
                    code: 0,
                    message: "success".to_string(),
                    data: "".to_string()
                })
            }
            Err(err) => Err(NaviError::DBError(err.to_string()))
        }
    }
}