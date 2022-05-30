use crate::models::navisites::*;
use log::info;

pub fn format_site_group(sites_data: &Vec<SearchSites>) -> Vec<ListNaviSites>{
    info!("format result data");
    let mut all_groups:Vec<(i32, String)> = vec![];
    for i in 0..sites_data.len() {
        if all_groups.contains(&(sites_data[i].group_id, sites_data[i].group_name.to_string())) {
            continue
        } else {
            all_groups.push((sites_data[i].group_id, sites_data[i].group_name.to_string()))
        }
    }
    // println!("all groups {:?}", all_groups);

    let mut inited_navi_site:Vec<ListNaviSites> = init_list_navi_sites(all_groups);

    for i in 0..sites_data.len() {
        for v in 0..inited_navi_site.len() {
            if sites_data[i].group_name == inited_navi_site[v].group_name {
                // inited_navi_site[v].group_id = sites_data[i].group_id;
                match &sites_data[i].site_name {
                    None => {
                        break
                    }
                    Some(_res) => {
                        // printl!("{:?} got item", inited_navi_site[v].group_name)
                        info!("{:?} got item", inited_navi_site[v].group_name)
                    }
                }
                inited_navi_site[v].navi_sites.push(NaviSiteRequest{
                    site_name: sites_data[i].site_name.clone(),
                    site_id: sites_data[i].site_id.unwrap(),
                    site_url: sites_data[i].site_url.clone(),
                    site_description: sites_data[i].site_description.clone(),
                    site_img_path: sites_data[i].site_img_path.clone(),
                })
            }
        }
    }
    // println!("{:?}", inited_navi_site);
    inited_navi_site
}


fn init_list_navi_sites(groups: Vec<(i32, String)>) -> Vec<ListNaviSites> {
    let mut navi_sites: Vec<ListNaviSites> = vec![];
    for i in 0..groups.len() {
        navi_sites.push(
            ListNaviSites {
                group_id: groups[i].0,
                group_name: groups[i].1.to_string(),
                navi_sites: vec![],
            }
        )
    }
    navi_sites
}