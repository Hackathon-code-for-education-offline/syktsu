use std::borrow::Cow;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct Response<'a> {
    code: usize,
    payload: Cow<'a, str>,
}
