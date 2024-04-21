#[macro_use]
extern crate rocket;

mod db;
mod jwt;

use app_interface::LoginRequest;
use db::{UniversityRow, DB};
use jsonwebtoken::{encode, EncodingKey};
use rocket::{data::FromData, fairing::{Fairing, Info, Kind}, http::{CookieJar, Header, Status}, serde::json::Json, Request, Response};
use serde::{Deserialize, Serialize};

pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new("Access-Control-Allow-Methods", "POST, GET, PATCH, OPTIONS"));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}

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
        .attach(CORS)
        .mount("/", routes![university, university_add, display_all, generate_token])
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct Claims {}

#[post("/login", format = "application/json", data = "<user>")]
fn generate_token(cookies: &CookieJar<'_>, user: LoginRequest) -> Status {
    // let key = EncodingKey::from_secret("secret".as_ref());
    // let token = encode(&header, &user, &key).unwrap();
    let user = user.clone().password;
    cookies.add_private(("session", user.clone()));

    Status::Ok
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
