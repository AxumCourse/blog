use serde::Deserialize;

/// 创建分类的表单
#[derive(Deserialize)]
pub struct CreateCategory {
    pub name:String,
}

/// 修改分类的表单
#[derive(Deserialize)]
pub struct EditCategory {
    pub id:i32,
    pub name:String,
}
