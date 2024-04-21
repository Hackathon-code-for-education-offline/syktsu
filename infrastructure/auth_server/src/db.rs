use serde::{de::DeserializeOwned, Deserialize, Serialize};
use sqlx::{query, MySql, MySqlPool, Pool};


const DB_NAME: &str = "test_db";
const ROOT_PASS: &str = "example";
const DB_USER: &str = "test_user";
const DB_PASS: &str = "test_pass";

pub struct DB {
    pub pool: Pool<MySql>,
    pub tables: Tables,
}

impl DB {
    pub async fn init() -> Self {
        let url = format!("mysql://root:{ROOT_PASS}@localhost:3306/{DB_NAME}");
        let pool = MySqlPool::connect(&url).await.unwrap();
        println!("Успешно подключен к базе данных!");

        let tables = Tables::init(&pool).await;

        Self {pool, tables}
    }
}

pub struct Tables {
    pub university: University,
    pub users: Users,
    pub roles: Roles,
    pub user_roles: UserRoles,
    pub reviews: Reviews,
    pub comments: Comments,
    pub locations: Locations,
    pub pictures: Pictures,
}

impl Tables {
    pub async fn init(pool: &Pool<MySql>) -> Self {
        let university = University::create(pool).await;
        let users = Users::create(pool).await;
        let roles = Roles::create(pool).await;
        let user_roles = UserRoles::create(pool).await;
        let reviews = Reviews::create(pool).await;
        let comments = Comments::create(pool).await;
        let locations = Locations::create(pool).await;
        let pictures = Pictures::create(pool).await;

        Self {
            university,
            users,
            roles,
            user_roles,
            reviews,
            comments,
            locations,
            pictures,
        }
    }
}

pub struct Roles {
    pool: Pool<MySql>
}
    
impl Roles {
    async fn create(pool: &Pool<MySql>) -> Self {
        if let Err(e) = query(r#"CREATE TABLE Roles (
            `role` VARCHAR(64) PRIMARY KEY
            );"#).execute(pool).await 
        {
            println!("{:?}", e);
        };

        Self {pool: pool.clone()}
    }

    async fn insert(pool: &Pool<MySql>) {
        if let Err(e) = query(r#""#).execute(pool).await {
            println!("{:?}", e);
        };
    }

    pub async fn select_by_key(&self, key: &str) -> String {
        let q = sqlx::query(&format!("SELECT {} FROM roles", key)).fetch_one(&self.pool).await.unwrap();
        format!("{:?}", q)  
    }
}

pub enum RolesRow {
    Guest,
    Student,
    Admin,
}


pub struct University {
    pool: Pool<MySql>
}

impl University {
    pub async fn create(pool: &Pool<MySql>) -> Self {
        if let Err(e) = query(r#"CREATE TABLE University (
            university_id INTEGER PRIMARY KEY AUTO_INCREMENT,
            title VARCHAR(256) NOT NULL,
            city VARCHAR(64),
            admission_committee VARCHAR(512)
            );"#).execute(pool).await 
        {
            println!("{:?}", e);
        };

        Self {pool: pool.clone()}
    }

    pub async fn insert(&self, row: UniversityRow) {
        if let Err(e) = query(&format!("insert into University (title, city) values ('{}', '{}');", row.title.unwrap(), row.city.unwrap())).execute(&self.pool).await {
            println!("{:?}", e);
        };
    }

    pub async fn select_by_key(&self, key: &str) -> String {
        let request = &format!("SELECT {} FROM University", key);
        let row = sqlx::query_as::<_, UniversityRow>(request).fetch_one(&self.pool).await.unwrap();
        
        row.into_json()
    }

    pub async fn select_all(&self) -> String {
        let request = &format!("SELECT * FROM University");
        let row = sqlx::query_as::<_, UniversityRow>(request).fetch_all(&self.pool).await.unwrap();
        
        serde_json::to_string(&row).unwrap()
    }
}

#[derive(Debug, sqlx::FromRow, Serialize, Deserialize)]
pub struct UniversityRow {
    university_id : i64,
    title: Option<String>,
    city: Option<String>,
    admission_committee: Option<String>,
}
impl InJson for UniversityRow {}

impl UniversityRow {
    pub fn new(title: &str, city: &str, comm_link: &str) -> Self {
        Self { university_id : 0, title: Some(title.to_string()), city: Some(city.to_string()), admission_committee: Some(comm_link.to_string()) }
    }
}

pub struct Users {
    pool: Pool<MySql>
}

impl Users {
    async fn create(pool: &Pool<MySql>) -> Self {
        if let Err(e) = query(r#"CREATE TABLE Users (
            user_id INTEGER PRIMARY KEY AUTO_INCREMENT,
            university_id INTEGER NOT NULL,
            last_name VARCHAR(128),
            first_name VARCHAR(128),
            middle_name VARCHAR(128),
            `role` VARCHAR(128) NOT NULL,
            institute VARCHAR(128),
            phone VARCHAR(16),
               FOREIGN KEY(university_id) REFERENCES University(university_id)
            );"#).execute(pool).await 
        {
            println!("{:?}", e);
        };

        Self {pool: pool.clone()}
    }

    async fn insert(pool: &Pool<MySql>) {
        if let Err(e) = query(r#""#).execute(pool).await {
            println!("{:?}", e);
        };
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UsersRow {
    id: i64,
    univer_id: i64,
    first_name: String,
    last_name: String,
    middle_name: String,
    // role: Roles,
    institute: String,
    phone: String,
}
impl InJson for UsersRow {}

pub struct UserRoles {
    pool: Pool<MySql>
}

impl UserRoles {
    async fn create(pool: &Pool<MySql>) -> Self {
        if let Err(e) = query(r#"CREATE TABLE UsersRoles (
            user_id INTEGER NOT NULL,
            `role` VARCHAR(64) NOT NULL,
            PRIMARY KEY(user_id, `role`),
            FOREIGN KEY (user_id) REFERENCES Users(user_id),
            FOREIGN KEY (`role`) REFERENCES Roles(`role`)
            );"#).execute(pool).await 
        {
            println!("{:?}", e);
        };

        Self {pool: pool.clone()}
    }

    async fn insert(pool: &Pool<MySql>) {
        if let Err(e) = query(r#""#).execute(pool).await {
            println!("{:?}", e);
        };
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserRolesRow {
    id: i64,
    // role: Roles,
}
impl InJson for UserRolesRow {}

pub struct Reviews {
    pool: Pool<MySql>
}

impl Reviews {
    async fn create(pool: &Pool<MySql>) -> Self {
        if let Err(e) = query(r#"CREATE TABLE Reviews (
            review_id INTEGER PRIMARY KEY AUTO_INCREMENT,
            user_id INTEGER NOT NULL,
            review TEXT NOT NULL,
            date_time DATETIME,
            evaluation INTEGER,
               FOREIGN KEY(user_id) REFERENCES Users(user_id)
           );"#).execute(pool).await 
        {
            println!("{:?}", e);
        };

        Self {pool: pool.clone()}
    }

    async fn insert(pool: &Pool<MySql>) {
        if let Err(e) = query(r#""#).execute(pool).await {
            println!("{:?}", e);
        };
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReviewsRow {
    id: i64,
    user_id: i64,
    review: String,
    date_time: usize,
    evaluation: i64,
}
impl InJson for ReviewsRow {}

pub struct Comments {
    pool: Pool<MySql>
}

impl Comments {
    async fn create(pool: &Pool<MySql>) -> Self {
        if let Err(e) = query(r#"CREATE TABLE Comments (
            user_id INTEGER NOT NULL,
            review_id INTEGER NOT NULL,
            `comment` TEXT NOT NULL,
            date_time DATETIME,
            PRIMARY KEY(user_id, review_id),
            FOREIGN KEY(user_id) REFERENCES Users(user_id),
            FOREIGN KEY(review_id) REFERENCES Reviews(review_id)
            );"#).execute(pool).await 
        {
            println!("{:?}", e);
        };

        Self {pool: pool.clone()}
    }

    async fn insert(pool: &Pool<MySql>) {
        if let Err(e) = query(r#""#).execute(pool).await {
            println!("{:?}", e);
        };
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CommentsRow {
    user_id: i64,
    review_id: i64,
    date_time: usize,
}
impl InJson for CommentsRow {}

pub struct Locations {
    pool: Pool<MySql>
}

impl Locations {
    async fn create(pool: &Pool<MySql>) -> Self {
        if let Err(e) = query(r#"CREATE TABLE Locations (
            location_id INTEGER PRIMARY KEY AUTO_INCREMENT,
            university_id INTEGER NOT NULL,
            title VARCHAR(128) NOT NULL,
            information TEXT,
            FOREIGN KEY(university_id) REFERENCES University(university_id)
            );"#).execute(pool).await {
            println!("{:?}", e);
        };

        Self {pool: pool.clone()}
    }

    async fn insert(pool: &Pool<MySql>) {
        if let Err(e) = query(r#""#).execute(pool).await {
            println!("{:?}", e);
        };
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LocationsRow {
    id: usize,
    title: String,
    info: String,
}
impl InJson for LocationsRow {}

pub struct Pictures {
    pool: Pool<MySql>
}

impl Pictures {
    async fn create(pool: &Pool<MySql>) -> Self {
        if let Err(e) = query(r#"CREATE TABLE Pictures (
            picture_id INTEGER PRIMARY KEY AUTO_INCREMENT,
            location_id INTEGER NOT NULL,
            pointer VARCHAR(512),
            FOREIGN KEY(location_id) REFERENCES Locations(location_id)
            );"#).execute(pool).await 
        {
            println!("{:?}", e);
        };

        Self {pool: pool.clone()}
    }

    async fn insert(pool: &Pool<MySql>) {
        if let Err(e) = query(r#""#).execute(pool).await {
            println!("{:?}", e);
        };
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PicturesRow {
    id: usize,
    pointer: String,
}
impl InJson for PicturesRow {}

pub trait InJson: Serialize + DeserializeOwned {
    fn into_json(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }

    fn from_json<T: DeserializeOwned>(raw: &str) -> T {
        serde_json::from_str(raw).unwrap()
    }
}