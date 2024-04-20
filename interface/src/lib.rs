use std::borrow::Cow;

#[derive(Clone, Debug)]
pub struct LoginRequest<'a> {
    username: Cow<'a, str>,
    password: Cow<'a, str>,
}
