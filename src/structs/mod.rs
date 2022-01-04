use chrono;
use serde::Deserialize;

pub type DateTime = chrono::DateTime<chrono::Utc>;

pub mod contract;
pub mod positions;
pub mod transaction;

#[derive(Deserialize, Debug)]
pub struct ListMetaResult {
    pub total_count: u32,
    pub next: Option<String>,
    pub previous: Option<String>,
    pub limit: u32,
    pub offset: u32,
}

#[derive(Deserialize, Debug)]
pub struct ListResult<T> {
    pub meta: ListMetaResult,
    pub data: T,
}