#[macro_use] extern crate rocket;

mod db;

use db::DB;
struct State {
    
}


impl State {
    pub async fn init() {
        let db = DB::init().await;
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

