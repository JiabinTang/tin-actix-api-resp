use crate::ApiErr;

impl std::error::Error for ApiErr {}

impl std::fmt::Display for ApiErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ApiErr: code={}, message={}",
            self.code(),
            self.message()
        )
    }
}

impl From<std::io::Error> for ApiErr {
    fn from(err: std::io::Error) -> Self {
        log::error!(target: "tin_actix_api_resp::error", "[IO] std::io::Error: {:#?}", err);
        ApiErr::InternalServerError("Internal server error".into())
    }
}

#[cfg(feature = "sea-orm")]
impl From<sea_orm::DbErr> for ApiErr {
    fn from(err: sea_orm::DbErr) -> Self {
        log::error!(target: "tin_actix_api_resp::error", "[SeaORM] DbErr: {:#?}", err);
        ApiErr::InternalServerError("Internal server error".into())
    }
}

#[cfg(feature = "sea-orm")]
impl From<sea_orm::TransactionError<ApiErr>> for ApiErr {
    fn from(err: sea_orm::TransactionError<ApiErr>) -> Self {
        match err {
            sea_orm::TransactionError::Transaction(err) => err,
            sea_orm::TransactionError::Connection(err) => {
                log::error!(target: "tin_actix_api_resp::error", "[SeaORM] TransactionError::Connection: {:#?}", err);
                ApiErr::InternalServerError("Internal server error".into())
            }
        }
    }
}

#[cfg(feature = "redis")]
impl From<redis::RedisError> for ApiErr {
    fn from(err: redis::RedisError) -> Self {
        log::error!(target: "tin_actix_api_resp::error", "[Redis] RedisError: {:#?}", err);
        ApiErr::InternalServerError("Internal server error".into())
    }
}

#[cfg(feature = "serde_json")]
impl From<serde_json::Error> for ApiErr {
    fn from(err: serde_json::Error) -> Self {
        log::error!(target: "tin_actix_api_resp::error", "[SerdeJson] serde_json::Error: {:#?}", err);
        ApiErr::InternalServerError("Internal server error".into())
    }
}

#[cfg(feature = "chrono")]
impl From<chrono::ParseError> for ApiErr {
    fn from(err: chrono::ParseError) -> Self {
        log::error!(target: "tin_actix_api_resp::error", "[Chrono] chrono::ParseError: {:#?}", err);
        ApiErr::InternalServerError("Internal server error".into())
    }
}
