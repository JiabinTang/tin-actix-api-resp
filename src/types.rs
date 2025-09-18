pub use crate::macros::ApiRes;
use crate::{EmptyRes, PageRes};

pub type ApiErr = ApiRes<()>;
pub type ApiResult<T> = Result<ApiRes<T>, ApiErr>;
pub type ApiBoolResult = Result<ApiRes<bool>, ApiErr>;
pub type ApiListResult<T> = Result<ApiRes<Vec<T>>, ApiErr>;
pub type ApiPageResult<T> = Result<ApiRes<PageRes<T>>, ApiErr>;
pub type ApiEmptyResult = Result<ApiRes<EmptyRes>, ApiErr>;
