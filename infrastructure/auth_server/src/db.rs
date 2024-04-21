use sqlx::{query, MySql, MySqlPool, Pool};


const DB_NAME: &str = "test_db";
const ROOT_PASS: &str = "example";
const DB_USER: &str = "test_user";
const DB_PASS: &str = "test_pass";

pub struct DB {
    pub pool: Pool<MySql>,
}

impl DB {
    pub async fn init() -> Self {
        let url = format!("mysql://root:{ROOT_PASS}@localhost:3306/{DB_NAME}");
        let pool = MySqlPool::connect(&url).await.unwrap();
        println!("Успешно подключен к базе данных!");

        Tables::init(&pool).await;

        Self {pool}
    }

    pub async fn show_table(&self, name: &str) {
        query(&format!("SELECT * FROM {};", name)).execute(&self.pool).await.unwrap();
    }
}

pub struct Tables {
    university: University,
    users: Users,
    roles: Roles,
    user_roles: UserRoles,
    reviews: Reviews,
    comments: Comments,
    locations: Locations,
    pictures: Pictures,
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

struct Roles;
    
impl Roles {
    async fn create(pool: &Pool<MySql>) -> Self {
        if let Err(e) = query(r#"CREATE TABLE Roles (
            `role` VARCHAR(64) PRIMARY KEY
            );"#).execute(pool).await 
        {
            println!("{:?}", e);
        };

        Self
    }

    async fn insert(pool: &Pool<MySql>) {
        if let Err(e) = query(r#""#).execute(pool).await {
            println!("{:?}", e);
        };
    }

    pub async fn select_by_key(pool: &Pool<MySql>, key: &str) -> String {
        sqlx::query(&format!("SELECT {} FROM roles", key)).fetch_one(pool).await.unwrap();
        String::new()        
    }
}

enum RolesRow {
    Guest,
    Student,
    Admin,
}


struct University;

impl University {
    async fn create(pool: &Pool<MySql>) -> Self {
        if let Err(e) = query(r#"CREATE TABLE University (
            university_id INTEGER PRIMARY KEY AUTO_INCREMENT,
            title VARCHAR(256) NOT NULL,
            city VARCHAR(64),
               admission_committee VARCHAR(512)
            );"#).execute(pool).await 
        {
            println!("{:?}", e);
        };

        Self
    }

    async fn insert(pool: &Pool<MySql>) {
        if let Err(e) = query(r#""#).execute(pool).await {
            println!("{:?}", e);
        };
    }
}

struct UniversityRow {
    id: i64,
    title: String,
    city: String,
}

struct Users;

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

        Self
    }

    async fn insert(pool: &Pool<MySql>) {
        if let Err(e) = query(r#""#).execute(pool).await {
            println!("{:?}", e);
        };
    }
}

struct UsersRow {
    id: i64,
    univer_id: i64,
    first_name: String,
    last_name: String,
    middle_name: String,
    role: Roles,
    institute: String,
    phone: String,
}

struct UserRoles;

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

        Self
    }

    async fn insert(pool: &Pool<MySql>) {
        if let Err(e) = query(r#""#).execute(pool).await {
            println!("{:?}", e);
        };
    }
}

struct UserRolesRow {
    id: i64,
    role: Roles,
}

struct Reviews;

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

        Self
    }

    async fn insert(pool: &Pool<MySql>) {
        if let Err(e) = query(r#""#).execute(pool).await {
            println!("{:?}", e);
        };
    }
}

struct ReviewsRow {
    id: i64,
    user_id: i64,
    review: String,
    date_time: usize,
    evaluation: i64,
}

struct Comments;

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

        Self
    }

    async fn insert(pool: &Pool<MySql>) {
        if let Err(e) = query(r#""#).execute(pool).await {
            println!("{:?}", e);
        };
    }
}

struct CommentsRow {
    user_id: i64,
    review_id: i64,
    date_time: usize,
}

struct Locations;

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

        Self
    }

    async fn insert(pool: &Pool<MySql>) {
        if let Err(e) = query(r#""#).execute(pool).await {
            println!("{:?}", e);
        };
    }
}

struct LocationsRow {
    id: usize,
    title: String,
    info: String,
}

struct Pictures;

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

        Self
    }

    async fn insert(pool: &Pool<MySql>) {
        if let Err(e) = query(r#""#).execute(pool).await {
            println!("{:?}", e);
        };
    }
}

struct PicturesRow {
    id: usize,
    pointer: String,
}