pub use crate::macros::ApiRes;
pub type ApiErr = ApiRes<()>;
pub type ApiResult<T> = Result<ApiRes<T>, ApiErr>;
