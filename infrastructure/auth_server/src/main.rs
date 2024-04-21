#[macro_use]
extern crate rocket;

mod db;
mod jwt;

use app_interface::{LoginRequest, Response};
use db::{UniversityRow, DB};
use jsonwebtoken::{encode, EncodingKey, Header};
use rocket::{data::FromData, http::{CookieJar, Status}, serde::json::Json};
use serde::{Deserialize, Serialize};

struct State {
    db: DB,
}

impl State {
    pub async fn init() -> Self {
        let db = DB::init().await;

        Self { db }
    }
}

#[launch]
async fn rocket() -> _ {
    let state = State::init().await;

    rocket::build()
        .manage(state)
        .mount("/", routes![university, university_add, display_all, generate_token])
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct Claims {}

#[post("/login", format = "application/json", data = "<user>")]
fn generate_token(cookies: &CookieJar<'_>, user: LoginRequest) -> Json<String> {
    let header = Header::default();
    let key = EncodingKey::from_secret("secret".as_ref());
    let token = encode(&header, &user, &key).unwrap();

    Json(token)
}

#[get("/university/add")]
async fn university_add(state: &rocket::State<State>) {
    let row = UniversityRow::new("CГУ", "Сыктывкар", "example.com");
    state.db.tables.university.insert(row).await;
}

#[get("/university/<id>")]
async fn university(state: &rocket::State<State>, id: String) -> String {
    state.db.tables.university.select_by_key(&id).await
}

#[get("/university/display")]
async fn display_all(state: &rocket::State<State>) -> String {
    state.db.tables.university.select_all().await
}
