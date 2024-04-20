use sqlx::*;
use sqlx::mysql::*;

const DB_NAME: &str = "test_db";
const DB_USER: &str = "test_user";
const DB_PASS: &str = "test_pass";

pub trait Data {}


pub struct User {
    id: i32,
    name: String,
}

impl User {
    pub fn new(id: i32, name: String) -> Self {
        Self {
            id,
            name
        }
    }
}

impl Data for User {}

pub struct DB {
    pool: Pool<MySql>
}

impl DB {
    pub async fn init() -> Self {
        let url = format!("mysql://{DB_USER}:{DB_PASS}@localhost:4444/{DB_NAME}");
        let pool = MySqlPoolOptions::new().connect(&url).await.unwrap();
        
        Self { 
            pool 
        }
    }

    pub async fn show_table(&self, name: &str) {
        query(&format!("SELECT * FROM {};", name)).execute(&self.pool).await.unwrap();
    }

    pub async fn insert(&self, name: &str, values: User) 
    {
        query(&format!(r"INSERT INTO {} VALUES (
            {}, {}
        );", 
            name,
            values.id,
            values.name
        )).execute(&self.pool).await.unwrap();
    }

    pub async fn create(&self, name: &str) {
        query(&format!(r"CREATE TABLE IF NOT EXIST {} (
            id int,
            name text,
        );", name)).execute(&self.pool).await.unwrap();
    }

    pub async fn select(&self) {
        query(r"").execute(&self.pool).await.unwrap();
    }
    
    pub async fn delete(&self) {
        query(r"").execute(&self.pool).await.unwrap();
    }
}

