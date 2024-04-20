#[macro_use] extern crate rocket;

mod db;

use db::{User, DB};
use rocket_db_pools::{
    sqlx::{self, Row, MySqlPool},
    Connection,
    Database,
};
use sqlx::query;
struct State {
    
}


impl State {
    pub async fn init() {
        let db = DB::init().await;
        println!("here!");

//         match query("CREATE TABLE University (university_id INTEGER PRIMARY KEY AUTO_INCREMENT, title VARCHAR(256) NOT NULL, city VARCHAR(64), admission_committee VARCHAR(512));").execute(&db.pool).await {
//             Ok(_) => println!("Таблица University создана"),
//             Err(e) => println!("{e}"),
//         };

//         query("INSERT INTO University(university_id, title) values (1, 'null');").execute(&db.pool).await.unwrap();
        
//         let r = query!(r#"
// SELECT * 
// FROM University
//                         "#)
//             .fetch_all(&db.pool).await.unwrap();

//         // let r = query("SELECT * FROM University").execute(&db.pool).await.unwrap();
//         println!("pinus {:?}", r);

//         for res in r {
//             println!("{}", res.university_id)
//         }
        
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

