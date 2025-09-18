use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageRes<T> {
    pub page: u64,
    pub size: u64,
    pub total: u64,
    pub list: Vec<T>,
}

impl<T> PageRes<T> {
    pub fn new(page: u64, size: u64, total: u64, list: Vec<T>) -> Self {
        Self {
            page,
            size,
            total,
            list,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmptyRes {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdRequest {
    pub id: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdsRequest {
    pub ids: Vec<i32>,
}
