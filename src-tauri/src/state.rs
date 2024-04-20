use std::error::Error;
use sqlx::mysql::*;

use crate::db::{User, DB};

const DB_NAME: &str = "test_db";
const DB_USER: &str = "test_user";
const DB_PASS: &str = "test_pass";

#[derive(Debug, PartialEq, Eq)]
struct Payment {
    customer_id: i32,
    amount: i32,
    account_name: Option<String>,
}



pub struct State {
    db: DB,
}

impl State {
    pub async fn init() -> Result<Self, Box<dyn Error>> {
        let db = DB::init().await;

        let table_name = "playground";

        db.create(table_name).await;
        db.insert(table_name, User::new(1, format!("Stiven"))).await;
        db.show_table(table_name).await;

        Ok(Self {
            db
        })
    }
}
