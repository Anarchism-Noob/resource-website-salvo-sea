use salvo::http::ParseError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("error:`{0}`")]
    AnyHow(#[from] anyhow::Error),
    #[error("http::ParseError:`{0}`")]
    ParseError(#[from] ParseError),
    #[error("sea_orm::DbErr:Error:`{0}`")]
    DbErr(#[from] sea_orm::DbErr),
    #[error("ValidationError:`{0}`")]
    ValidationError(#[from] validator::ValidationErrors),
}