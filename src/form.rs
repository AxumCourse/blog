use serde::Deserialize;

/// 创建分类的表单
#[derive(Deserialize)]
pub struct CreateCategory {
    pub name: String,
}

/// 修改分类的表单
#[derive(Deserialize)]
pub struct EditCategory {
    pub id: i32,
    pub name: String,
}

/// 创建文章的表单
#[derive(Deserialize)]
pub struct CreateTopic {
    pub title: String,
    pub category_id: i32,
    pub summary: String,
    pub markdown: String,
}


pub type EditTopic = CreateTopic;

#[derive(Deserialize)]
pub struct AdminLogin {
    pub email:String,
    pub password:String,
}
