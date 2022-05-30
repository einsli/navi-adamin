use std::collections::HashMap;
use mysql::{Error, Pool as mysqlPool};
use crate::models::navisites::SelectDuplicatedSite;
use mysql::QueryResult;
use crate::error::NaviError;
use crate::response::NaviBaseResponse;

pub fn check_url_existed(pool: &mysqlPool, site_url: String) -> bool {
    let navi_sites: Vec<SelectDuplicatedSite> = pool.prep_exec(
        "SELECT site_url FROM navi_sites WHERE is_deleted = 0 and site_url = ?",
        (site_url,)
    ).map(|result| {
        result
            .map(|x| x.unwrap())
            .map(|row| {
                let site_url = mysql::from_row(row);
                SelectDuplicatedSite {
                    site_url: site_url
                }
            })
            .collect()
    })
        .unwrap();
    match navi_sites.len() {
        0 => {false},
        _ => { true }
    }
}


pub fn create_sites_res(res: Result<QueryResult, Error>)
                        -> Result<NaviBaseResponse<String>, NaviError> {
    match res {
        Ok(site_res) => if site_res.affected_rows() == 0 {
            Err(NaviError::NotFound("No sites saved!".to_string()))
        } else {
            let mut inserted_map: HashMap<String, u64> = HashMap::new();
            inserted_map.insert("site_id".to_string(), site_res.last_insert_id());

            Ok(NaviBaseResponse{
                code: 0,
                message: "success".to_string(),
                data: "".to_string(),
            })
        }
        Err(err) => Err(NaviError::DBError(err.to_string()))
    }
}

pub fn delete_db_res(delete_res: Result<QueryResult,Error>)
                     -> Result<NaviBaseResponse<String>, NaviError> {

    match delete_res {
        Ok(res) => if res.affected_rows() == 0 {
            Err(NaviError::NotFound("No data found".into()))
        } else {
            Ok(NaviBaseResponse{
                code: 0,
                message:  "success".to_string(),
                data: "".to_string(),
            })
        }
        Err(err) => Err(NaviError::NotFound(err.to_string()))
    }
}