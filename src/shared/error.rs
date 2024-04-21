use std::borrow::Cow;

use serde::{Deserialize, Serialize};
use thiserror::Error;
use wasm_bindgen::JsValue;

#[derive(Error, Debug, Serialize, Deserialize, Clone)]
pub enum UiError<'a> {
    #[error("an error occured at client-side: {0}")]
    JsError(Cow<'a, str>),
    #[error("reqwest error occured: {0}")]
    ReqwestError(Cow<'a, str>),
    #[error("unknown client-side error")]
    Unknown,
}

impl<'a> From<JsValue> for UiError<'a> {
    fn from(value: JsValue) -> Self {
        Self::JsError(Cow::Owned(
            serde_wasm_bindgen::from_value::<String>(value).unwrap_or_default(),
        ))
    }
}

impl<'a> From<reqwest::Error> for UiError<'a> {
    fn from(value: reqwest::Error) -> Self {
        Self::ReqwestError(Cow::Owned(value.to_string()))
    }
}
