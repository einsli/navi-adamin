// 保证线程是安全的
use std::sync::Mutex;
use mysql::Pool as mysqlPool;

pub struct AppState {
    pub health_check_response: String,
    pub visit_count: Mutex<u32>,  // 可变的
    pub db: mysqlPool,// 可以做多线程里面共享这个连接池
}