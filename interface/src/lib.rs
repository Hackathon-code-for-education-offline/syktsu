#[cfg(feature = "backend")]
use rocket::{
    data::{self, FromData, Outcome, ToByteUnit},
    http::{ContentType, Status},
    request, Data, FromForm, Request,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::{borrow::Cow, collections::HashMap};

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
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[cfg(feature = "backend")]
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

        Outcome::Success(LoginRequest {
            username: format!("{}", username),
            password: format!("{}", password),
        })
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct University {
    pub id: usize,
    pub title: String,
    pub country: String,
    pub score: f32,
    pub voters: usize,
    #[serde(rename(deserialize = "linkProfile"))]
    pub link_profile: String,
    #[serde(rename(deserialize = "linkPic"))]
    pub link_pic: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub map: Option<PannellumOptions>,
}

#[derive(Serialize, Default, PartialEq, Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PannellumOptions {
    pub default: PannellumDefault,
    pub scenes: HashMap<String, PannellumScene>,
}

#[derive(Serialize, Deserialize, Default, PartialEq, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PannellumDefault {
    #[serde(rename(deserialize = "firstScene"))]
    pub first_scene: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename(deserialize = "sceneFadeDuration"))]
    pub scene_fade_duration: Option<usize>,
}

#[derive(Serialize, Deserialize, Default, PartialEq, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PannellumScene {
    #[serde(rename = "type")]
    #[serde(rename(deserialize = "type"))]
    pub _type: String,
    pub panorama: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename(deserialize = "hotSpots"))]
    pub hot_spots: Option<Vec<HotSpot>>,
}

#[derive(Serialize, Deserialize, Default, Clone, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct HotSpot {
    pub pitch: f64,
    pub yaw: f64,
    #[serde(rename = "type")]
    #[serde(rename(deserialize = "type"))]
    pub _type: String,
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename(deserialize = "sceneId"))]
    pub scene_id: Option<String>,
}

#[test]
fn convert() {
    let test = json!({
      "default": {
          "firstScene": "circle"
      },

      "scenes": {
          "circle": {
              "type": "equirectangular",
              "panorama": "/images/from-tree.jpg",
              "hotSpots": [
                  {
                      "pitch": -2.1,
                      "yaw": 132.9,
                      "type": "scene",
                      "text": "Spring House or Dairy",
                      "sceneId": "house"
                  }
              ]
          },

          "house": {
              "title": "Spring House or Dairy",
              "type": "equirectangular",
              "panorama": "/images/bma-0.jpg",
              "hotSpots": [
                  {
                      "pitch": -0.6,
                      "yaw": 37.1,
                      "type": "scene",
                      "text": "Mason Circle",
                      "sceneId": "circle"
                  }
              ]
          }
      }
    });

    println!(
        "{:?}",
        serde_json::from_value::<PannellumOptions>(test).is_err()
    );
}
