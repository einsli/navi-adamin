use std::fmt::Debug;
use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
pub struct NaviBaseResponse<T>
    where T:Serialize+Debug
{
    pub code: i32,
    pub message: String,
    pub data: T
}

