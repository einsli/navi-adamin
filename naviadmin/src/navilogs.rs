use log::info;
use log4rs;

pub fn init_navi_logs() {
    log4rs::init_file("config/log4rs.yaml", Default::default()).unwrap();
    info!("Init Navi Log Successfully!");
}