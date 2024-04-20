use super::error::UiError;

pub type Result<T> = std::result::Result<T, UiError<'static>>;