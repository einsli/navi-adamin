use actix_web::web;
use actix_web::web::Json;
use serde::{Deserialize, Serialize};
use crate::error::NaviError;
use std::convert::{TryFrom};


/*
mysql db table model
*/
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct NaviGroup {
    pub id: i32,
    pub name: Option<String>,
    pub is_deleted: i32,
}


/*
request model
*/
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ListNaviGroup{
    pub id: i32,
    pub name: Option<String>
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateNaviGroup {
    pub name: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct UpdateNaviGroup {
    pub id: i32,
    pub name: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct QueryNaviGroup {
    pub id: i32,
    pub name: Option<String>,
}

impl TryFrom<web::Json<CreateNaviGroup>> for CreateNaviGroup {
    type Error = NaviError;

    fn try_from(navi_group: Json<CreateNaviGroup>) -> Result<Self, Self::Error>{
        Ok(CreateNaviGroup {
            name: navi_group.name.clone(),
        })
    }
}

impl TryFrom<web::Json<UpdateNaviGroup>> for UpdateNaviGroup {
    type Error = NaviError;

    fn try_from(navi_group: Json<UpdateNaviGroup>) -> Result<Self, Self::Error>{
        Ok(UpdateNaviGroup {
            id: navi_group.id,
            name: navi_group.name.clone(),
        })
    }
}