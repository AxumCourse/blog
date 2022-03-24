use serde::{Deserialize, Serialize};

/// 分页对象
#[derive(Deserialize, Serialize)]
pub struct Paginate<T> {
    /// 当前页码
    pub page: u32,
    /// 分页大小
    pub page_size: u8,
    /// 总记录数
    pub total_records: i64,
    /// 分页数
    pub total_pages: i64,
    /// 数据
    pub data: T,
}

impl<T> Paginate<T> {
    /// 创建一个新的分页对象
    pub fn new(page: u32, page_size: u8, total_records: i64, data: T) -> Self {
        let total_pages = f64::ceil(total_records as f64 / page_size as f64) as i64;
        Self {
            page,
            page_size,
            total_records,
            total_pages,
            data,
        }
    }
}
