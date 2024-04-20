#[macro_use] extern crate rocket;

mod db;
mod jwt;

use db::DB;
use rocket::serde::json::Json;
use serde::Deserialize;
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

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
struct TokenInData {

}

#[post("/gen_token", data = "<token_data>")]
fn generate_token(token_data: Json<TokenInData>) {
    
}