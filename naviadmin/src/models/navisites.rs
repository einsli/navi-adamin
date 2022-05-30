use actix_web::web;
use actix_web::web::Json;
use serde::{Deserialize, Serialize};
use crate::error::NaviError;
use std::convert::{TryFrom};

/*
mysql db table model
*/
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct NaviSite {
    pub id: i32,
    pub name: Option<String>,
    pub site_url: Option<String>,
    pub site_description: Option<String>,
    pub site_img_path: Option<String>,
    pub site_group_id: i32,
    pub is_deleted: i32,
}


/*
request model
*/

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct NaviSiteRequest {
    pub site_id: i32,
    pub site_name: Option<String>,
    pub site_url: Option<String>,
    pub site_description: Option<String>,
    pub site_img_path: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ListNaviSites {
    pub group_id: i32,
    pub group_name: String,
    pub navi_sites: Vec<NaviSiteRequest>
}

#[derive(Serialize, Debug, Clone)]
pub struct SearchSites {
    pub group_id: i32,
    pub site_id: Option<i32>,
    pub group_name: String,
    pub site_name: Option<String>,
    pub site_url:Option<String>,
    pub site_description: Option<String>,
    pub site_img_path:Option<String>
}


#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct CreateNaviSite {
    pub name: Option<String>,
    pub site_url: Option<String>,
    pub site_description: Option<String>,
    pub image_path: Option<String>,
    pub site_group_id: i32,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct UpdateNaviSite {
    pub name: Option<String>,
    pub site_url: Option<String>,
    pub site_description: Option<String>,
    pub image_path: Option<String>,
    pub site_group_id: i32,
}

#[derive(Serialize, Debug, Clone)]
pub struct SelectDuplicatedSite {
    pub site_url: Option<String>
}

impl TryFrom<web::Json<CreateNaviSite>> for CreateNaviSite {
    type Error = NaviError;

    fn try_from(navi_site: Json<CreateNaviSite>) -> Result<Self, Self::Error>{
        Ok(CreateNaviSite {
            name: navi_site.name.clone(),
            site_url: navi_site.site_url.clone(),
            site_description: navi_site.site_description.clone(),
            image_path: navi_site.image_path.clone(),
            site_group_id: navi_site.site_group_id,
        })
    }
}

impl TryFrom<web::Json<UpdateNaviSite>> for UpdateNaviSite {
    type Error = NaviError;

    fn try_from(navi_site: Json<UpdateNaviSite>) -> Result<Self, Self::Error>{
        Ok(UpdateNaviSite {
            name: navi_site.name.clone(),
            site_url: navi_site.site_url.clone(),
            site_description: navi_site.site_description.clone(),
            image_path: navi_site.image_path.clone(),
            site_group_id: navi_site.site_group_id,
        })
    }
}