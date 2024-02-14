use thiserror::Error;

#[derive(Error, Debug)]
pub enum DiveErr {
    #[error("diesel Error")]
    IoErr(#[from] diesel::result::Error),
    #[error("r2d2 Error")]
    R2d2Err(#[from] r2d2::Error),
    #[error("dotenv Error")]
    DotenvError(#[from] dotenvy::Error),
    #[error("uuid Erro")]
    UuidError(#[from] uuid::Error),
}
