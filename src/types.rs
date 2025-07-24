use serde::Serialize;

pub use crate::macros::ApiRes;
pub type ApiErr = ApiRes<()>;
pub type ApiResult<T> = Result<ApiRes<T>, ApiErr>;
pub type ApiPageRes<T> = ApiRes<PageRes<T>>;

#[derive(Serialize, Debug)]
pub struct PageRes<T> {
    pub page: u64,
    pub per_page: u64,
    pub total: u64,
    pub list: Vec<T>,
}
