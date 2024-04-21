#[cfg(feature = "backend")]
use rocket::{
    data::{self, ToByteUnit, FromData, Outcome}, 
    FromForm, 
    http::{ContentType, Status}, 
    request, 
    Data, 
    Request
};
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

// #[derive(Clone, Debug, Serialize, Deserialize, FromForm)]
// pub struct LoginRequest<'a> {
//     pub username: Cow<'a, str>,
//     pub password: Cow<'a, str>,
// }

#[derive(Clone, Debug, Serialize, Deserialize, FromForm)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug)]
pub enum Error {
    TooLarge,
    NoColon,
    InvalidAge,
    Io(std::io::Error),
}

// #[cfg(feature = "backend")]
// #[rocket::async_trait]
// impl<'r> FromData<'r> for LoginRequest<'r> {
//     type Error = Error;

//     async fn from_data(req: &'r Request<'_>, data: Data<'r>) -> data::Outcome<'r, Self> {
//         use Error::*;

//         // Ensure the content type is correct before opening the data.
//         let person_ct = ContentType::new("application", "x-person");
//         if req.content_type() != Some(&person_ct) {
//             return Outcome::Forward((data, Status::UnsupportedMediaType));
//         }

//         // Use a configured limit with name 'person' or fallback to default.
//         let limit = req.limits().get("person").unwrap_or(256.bytes());

//         // Read the data into a string.
//         let string = match data.open(limit).into_string().await {
//             Ok(string) if string.is_complete() => string.into_inner(),
//             Ok(_) => return Outcome::Error((Status::PayloadTooLarge, TooLarge)),
//             Err(e) => return Outcome::Error((Status::InternalServerError, Io(e))),
//         };

//         // We store `string` in request-local cache for long-lived borrows.
//         let string = request::local_cache!(req, string);

//         // Split the string into two pieces at ':'.
//         let (username, password) = match string.find(':') {
//             Some(i) => (&string[..i], &string[(i + 1)..]),
//             None => return Outcome::Error((Status::UnprocessableEntity, NoColon)),
//         };

//         Outcome::Success(LoginRequest { username: Cow::Owned(format!("{}", username)), password: Cow::Owned(format!("{}", password)) })
//     }
// }

#[cfg(feature = "backend")]
#[rocket::async_trait]
impl<'r> FromData<'r> for LoginRequest {
    type Error = Error;

    async fn from_data(req: &'r Request<'_>, data: Data<'r>) -> data::Outcome<'r, Self> {
        use Error::*;

        // Ensure the content type is correct before opening the data.
        let person_ct = ContentType::new("application", "x-person");
        if req.content_type() != Some(&person_ct) {
            return Outcome::Forward((data, Status::UnsupportedMediaType));
        }

        // Use a configured limit with name 'person' or fallback to default.
        let limit = req.limits().get("person").unwrap_or(256.bytes());

        // Read the data into a string.
        let string = match data.open(limit).into_string().await {
            Ok(string) if string.is_complete() => string.into_inner(),
            Ok(_) => return Outcome::Error((Status::PayloadTooLarge, TooLarge)),
            Err(e) => return Outcome::Error((Status::InternalServerError, Io(e))),
        };

        // We store `string` in request-local cache for long-lived borrows.
        let string = request::local_cache!(req, string);

        // Split the string into two pieces at ':'.
        let (username, password) = match string.find(':') {
            Some(i) => (&string[..i], &string[(i + 1)..]),
            None => return Outcome::Error((Status::UnprocessableEntity, NoColon)),
        };

        Outcome::Success(LoginRequest { username: format!("{}", username), password: format!("{}", password) })
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct University<'a> {
    pub id: usize,
    pub title: Cow<'a, str>,
    pub country: Cow<'a, str>,
    pub score: f32,
    pub voters: usize,
    pub link_profile: Cow<'a, str>,
    pub link_pic: Cow<'a, str>,
}
