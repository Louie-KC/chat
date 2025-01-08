use log::warn;
use sqlx::{
    MySql,
    Pool
};
use sqlx::mysql::MySqlPoolOptions;

type DBResult<T> = Result<T, DatabaseServiceError>;

pub enum DatabaseServiceError {
    NoResult,
    KeyAlreadyExists,
    SQLXError(sqlx::Error)
}

impl From<sqlx::Error> for DatabaseServiceError {
    fn from(value: sqlx::Error) -> Self {
        let err = match value {
            sqlx::Error::RowNotFound => DatabaseServiceError::NoResult,
            _ => DatabaseServiceError::SQLXError(value)
        };
        warn!("{}", err);
        err
    }
}

impl std::fmt::Display for DatabaseServiceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let output= match self {
            DatabaseServiceError::NoResult => "No result".to_string(),
            DatabaseServiceError::KeyAlreadyExists => "Key already exists".to_string(),
            DatabaseServiceError::SQLXError(error) => error.to_string(),
        };
        write!(f, "{}", output)
    }
}

pub struct DatabaseService {
    conn_pool: Pool<MySql>
}

impl DatabaseService {
    /// Create and initialise the database service, connecting to the MySQL database
    /// at the provided `url`.
    /// 
    /// # Panics
    /// A panic occurs if a connection cannot be established.
    pub async fn new(url: &str) -> Self {
        let pool = MySqlPoolOptions::new().connect(url)
            .await
            .expect("Failed to connect to the database");

        DatabaseService { conn_pool: pool }
    }

    /// Test the current connection to the database by performing a simple query.
    pub async fn health_check(&self) -> DBResult<()> {
        match sqlx::query("SELECT 1;").execute(&self.conn_pool).await {
            Ok(_) => Ok(()),
            Err(err) => Err(err.into()),
        }
    }

}
