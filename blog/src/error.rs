// #[derive(thiserror::Error, Debug)]
pub enum ApiError {
    #[error("internal error")]
    InternalError,
    #[error("bad request")]
    BadRequest,
    #[error("not found")]
    NotFound,
}

/*
- https://zenn.dev/a24k/articles/20221113-wasmple-define-macros#%E5%AE%A3%E8%A8%80%E3%83%9E%E3%82%AF%E3%83%AD
- https://doc.rust-jp.rs/rust-by-example-ja/macros.html
 */
macro_rules! impl_from_trait {
    ($error:ty) => {
        impl From<$error> for ApiError {
            fn from(e: $etype) -> Self {
                ApiError::Other(anyhow::anyhow!(e))
            }
        }
    };
}

impl_from_trait!(diesel::result::Error);
impl_from_trait!(diesel::r2d2::PoolError);
impl_from_trait!((diesel::r2d2::Error));
impl_from_trait!(actix_web::error::BlockingError);
