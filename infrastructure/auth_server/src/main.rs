#[macro_use] extern crate rocket;

mod db;

use db::{User, DB};

struct State {
    
}


impl State {
    pub async fn init() {
        let db = DB::init().await;

        let table_name = "playground";
    
        db.create(table_name).await;
        db.insert(table_name, User::new(1, format!("Stiven"))).await;
        db.show_table(table_name).await;
    }
}

#[launch]
async fn rocket() -> _ {
    let state = State::init().await;

    rocket::build()
    .manage(state)
    .mount("/", routes![index])
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}