use axum::response::IntoResponse;

/// 应用程序错误类型
#[derive(Debug)]
pub enum AppErrorType {
    Db,
    Template,
    Notfound,
}

/// 应用程序错误
#[derive(Debug)]
pub struct AppError {
    pub message: Option<String>,
    pub cause: Option<Box<dyn std::error::Error>>,
    pub types: AppErrorType,
}

impl AppError {
    fn new(message:Option<String>, cause:Option<Box<dyn std::error::Error>>, types: AppErrorType) -> Self {
        Self { message, cause, types}
    }
    fn from_err(cause:Box<dyn std::error::Error>, types: AppErrorType) -> Self {
        Self::new(None, Some(cause), types)
    }
    fn from_str(msg:&str, types:AppErrorType) ->Self {
        Self::new(Some(msg.to_string()), None, types)
    }
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for AppError{}


impl From<deadpool_postgres::PoolError> for AppError {
   fn from(err: deadpool_postgres::PoolError) -> Self {
      Self::from_err(Box::new(err), AppErrorType::Db)
   }
}

impl From<tokio_pg_mapper::Error> for AppError {
    fn from(err: tokio_pg_mapper::Error) -> Self {
        Self::from_err(Box::new(err), AppErrorType::Db)
    }
}

impl From<askama::Error> for AppError {
    fn from(err: askama::Error) -> Self {
        Self::from_err(Box::new(err), AppErrorType::Template)
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let msg = match self.message {
            Some(msg) => msg.clone(),
            None => "有错误发生".to_string(),
        };
        msg.into_response()
    }
}
