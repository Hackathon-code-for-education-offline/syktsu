use std::borrow::Cow;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LoginRequest<'a> {
    username: Cow<'a, str>,
    password: Cow<'a, str>,
}
