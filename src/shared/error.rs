use std::borrow::Cow;

use thiserror::Error;
use serde::{Serialize, Deserialize};
use wasm_bindgen::JsValue;

#[derive(Error, Debug, Serialize, Deserialize)]
pub enum UiError<'a> {
    #[error("an error occured at client-side")]
    JsError(Cow<'a, str>),
    #[error("unknown client-side error")]
    Unknown,
}


impl<'a> From<JsValue> for UiError<'a> {
    fn from(value: JsValue) -> Self {
        Self::JsError(
            Cow::Owned(serde_wasm_bindgen::from_value::<String>(value)
                .unwrap_or_default())
        )
    }
}