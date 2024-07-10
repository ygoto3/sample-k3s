use std::env;
use once_cell::sync::OnceCell;
use sea_orm::{Database, DatabaseConnection, DbErr};

pub struct DatabaseSession {
    pub connection: DatabaseConnection,
}

impl DatabaseSession {

    pub async fn new(db_url: String, db_user: String, db_password: String) -> Result<Self, DbErr> {
        let db_connection = Database::connect(format!("mysql://{}:{}@{}", db_user, db_password, db_url)).await?;
        Ok(Self { connection: db_connection })
    }

}

pub static DATABASE_SESSION: OnceCell<DatabaseSession> = OnceCell::new();
pub static DATABASE_READ_SESSION: OnceCell<DatabaseSession> = OnceCell::new();

pub async fn init_session() -> Result<(), String> {
    let env_urls = ["DATABASE_URL", "DATABASE_READ_URL"];
    for env_url in env_urls.iter() {
        let db_url = env::var(env_url).unwrap_or_else(|_| "127.0.0.1:3306/db".to_string());
        let db_user = env::var("DATABASE_USER").unwrap_or_else(|_| "mysql".to_string());
        let db_password = env::var("DATABASE_PASSWORD").unwrap_or_else(|_| "mysqldevpassword".to_string());
        let db_session = match DatabaseSession::new(db_url, db_user, db_password).await {
            Ok(session) => session,
            Err(err) => return Err(format!("Failed to connect to database: {:?}", err)),
        };
        if *env_url == env_urls[0] {
            DATABASE_SESSION.set(db_session).map_err(|_| "Failed to set database session".to_string())?;
        } else {
            DATABASE_READ_SESSION.set(db_session).map_err(|_| "Failed to set database read session".to_string())?;
        }
    }
    Ok(())
}
