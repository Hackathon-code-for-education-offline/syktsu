#[macro_use] extern crate rocket;

mod db;
mod jwt;

use db::DB;
use jsonwebtoken::{encode, EncodingKey, Header};
use rocket::serde::json::Json;
use serde::{Serialize, Deserialize};
struct State {
    
}


impl State {
    pub async fn init() {
        let _ = DB::init().await;
    }
}


#[launch]
async fn rocket() -> _ {
    let state = State::init().await;

    rocket::build()
        .manage(state)
        .mount("/", routes![
            // read
        ])
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct Claims {

}

#[post("/gen_token", data = "<claims>")]
fn generate_token(claims: Json<Claims>) -> String {
    let claims = claims.0;

    let header = Header::default();
    let key = EncodingKey::from_secret("secret".as_ref());
    let token = encode(&header, &claims, &key).unwrap();

    token
}

#[post("/university/<id>")]
fn university(id: String) -> String {
    id
}