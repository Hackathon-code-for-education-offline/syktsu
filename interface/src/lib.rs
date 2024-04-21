use std::borrow::Cow;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LoginRequest<'a> {
    pub username: Cow<'a, str>,
    pub password: Cow<'a, str>,
}
