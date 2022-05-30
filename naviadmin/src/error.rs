use actix_web::{error, http::StatusCode, HttpResponse, Result};
use serde::Serialize;
use std::fmt;
use actix_web::error::BlockingError;
use mysql::error::Error as MySQLError;
use std::io::Error as IOError;
use actix_multipart::MultipartError;

#[derive(Debug, Serialize)]
pub enum NaviError {
    DBError(String),
    ActixError(String),
    NotFound(String),
    InvalidInput(String),
    StdIoError(String),
    StdBlockingError(String),
    NaviMultiPartError(String),
}

#[derive(Debug, Serialize)]
pub struct NaviErrorResponse {
    code: i32,
    message: String,
}

impl NaviError {
    fn error_response(&self) -> String {
        match self {
            NaviError::DBError(msg) => {
                println!("Database error occurred: {:?}", msg);
                "Database error".into()
            }
            NaviError::ActixError(msg) => {
                println!("Server error occurred: {:?}", msg);
                "Internal server error".into()
            }
            NaviError::NotFound(msg) => {
                println!("Not Found error: {:?}", msg);
                "Not Found".into()
            }
            NaviError::InvalidInput(msg) => {
                println!("Invalid parameters received: {:?}", msg);
                msg.into()
            }
            NaviError::StdBlockingError(msg) => {
                println!("Blocking error: {:?}", msg);
                msg.into()
            }
            NaviError::StdIoError(msg) => {
                println!("Io error: {:?}", msg);
                msg.into()
            }
            NaviError::NaviMultiPartError(msg) => {
                println!("MultiPart error: {:?}", msg);
                msg.into()
            }
        }
    }
}

impl error::ResponseError for NaviError {
    fn status_code(&self) -> StatusCode {
        match self {
            NaviError::DBError(_msg) | NaviError::ActixError(_msg) => StatusCode::INTERNAL_SERVER_ERROR,
            NaviError::NotFound(_msg) => StatusCode::NOT_FOUND,
            NaviError::InvalidInput(_msg) => StatusCode::BAD_REQUEST,
            NaviError::StdBlockingError(_msg) => StatusCode::INTERNAL_SERVER_ERROR,
            NaviError::StdIoError(_msg) => StatusCode::INTERNAL_SERVER_ERROR,
            NaviError::NaviMultiPartError(_msg) => StatusCode::BAD_REQUEST,
        }
    }
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(NaviErrorResponse {
            code: -1,
            message: self.error_response(),
        })
    }
}

impl fmt::Display for NaviError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}", self)
    }
}

impl From<actix_web::error::Error> for NaviError {
    fn from(err: actix_web::error::Error) -> Self {
        NaviError::ActixError(err.to_string())
    }
}

impl From<MySQLError> for NaviError {
    fn from(err: MySQLError) -> Self {
        NaviError::DBError(err.to_string())
    }
}

impl From<BlockingError> for NaviError {
    fn from(err: BlockingError) -> Self {
        NaviError::StdBlockingError(err.to_string())
    }
}

impl From<IOError> for NaviError {
    fn from(err: IOError) -> Self {
        NaviError::StdIoError(err.to_string())
    }
}

impl From<MultipartError> for NaviError {
    fn from(err: MultipartError) -> Self {
        NaviError::NaviMultiPartError(err.to_string())
    }
}