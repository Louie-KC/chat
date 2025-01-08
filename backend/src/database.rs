use log::warn;
use sqlx::{
    MySql,
    Pool
};
use sqlx::mysql::MySqlPoolOptions;
use uuid::Uuid;

use crate::models::DBUser;

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

    /// Determine if a User record exists in the connected database with the
    /// provided `username`.
    pub async fn user_exists(&self, username: &str) -> DBResult<bool> {
        let qr = sqlx::query!(
            "SELECT COUNT(*) as count
            FROM User
            WHERE username = ?",
            username)
            .fetch_one(&self.conn_pool)
            .await;

        match qr {
            Ok(r) => Ok(r.count > 0),
            Err(err) => Err(err.into())
        }
    }

    /// Record a new User in the connected database.
    pub async fn user_register(&self, username: &str, password_hash: String) -> DBResult<()> {
        let qr = sqlx::query!(
            "INSERT INTO User (username, password_hash) VALUES (?, ?);",
            username,
            password_hash)
            .execute(&self.conn_pool)
            .await;

        match qr {
            Ok(_)  => Ok(()),
            Err(err) => Err(err.into()),
        }
    }

    /// Retrieve the User record from the connected database with the provided
    /// `username`.
    pub async fn user_get(&self, username: &str) -> DBResult<DBUser> {
        let qr = sqlx::query_as!(
            DBUser,
            "SELECT *
            FROM User
            WHERE username = ?;",
            username)
            .fetch_one(&self.conn_pool)
            .await;

        Ok(qr?)
    }

    /// Create an entry in the UserToken table, mapping an auth `token` to a
    /// `user_id` granting authorization.
    pub async fn user_set_token(&self, user_id: &u64, token: &Uuid) -> DBResult<()> {
        let qr = sqlx::query!(
            "INSERT INTO UserToken (token, user_id) VALUES (?, ?);",
            token.to_string(),
            user_id)
            .execute(&self.conn_pool)
            .await;

        match qr {
            Ok(r) if r.rows_affected() > 0 => Ok(()),
            Ok(_)  => Err(DatabaseServiceError::KeyAlreadyExists),
            Err(e) => Err(e.into()),
        }
    }

}