use serde::Serialize;

pub use crate::macros::ApiRes;

pub type ApiErr = ApiRes<()>;
pub type ApiResult<T> = Result<ApiRes<T>, ApiErr>;
pub type ApiBoolResult = Result<ApiRes<bool>, ApiErr>;
pub type ApiListResult<T> = Result<ApiRes<Vec<T>>, ApiErr>;
pub type ApiPageResult<T> = Result<ApiRes<PageRes<T>>, ApiErr>;

#[derive(Serialize, Debug)]
pub struct PageRes<T> {
    pub page: u64,
    pub per_page: u64,
    pub total: u64,
    pub list: Vec<T>,
}
