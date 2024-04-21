use serde::{Deserialize, Serialize};
use std::borrow::Cow;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Response<T> {
    pub code: Code,
    pub payload: T,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Code {
    #[serde(rename = "200")]
    Success,
    #[serde(rename = "500")]
    InternalServerError,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LoginRequest<'a> {
    pub username: Cow<'a, str>,
    pub password: Cow<'a, str>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct University<'a> {
    pub link_profile: Cow<'a, str>,
    pub link_pic: Cow<'a, str>,
}
