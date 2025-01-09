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

    /*  User management  */

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
    pub async fn user_get_by_username(&self, username: &str) -> DBResult<DBUser> {
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

    /// Retrieve the User record from the connected database with the provided
    /// `username`.
    pub async fn user_get_by_id(&self, user_id: &u64) -> DBResult<DBUser> {
        let qr = sqlx::query_as!(
            DBUser,
            "SELECT *
            FROM User
            WHERE id = ?;",
            user_id)
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

    /// Find the user_id associated with the provided `token` (if present).
    pub async fn user_id_from_token(&self, token: &Uuid) -> DBResult<u64> {
        let qr = sqlx::query!(
            "SELECT user_id
            FROM UserToken
            WHERE token = ?",
            token.to_string()
        ).fetch_one(&self.conn_pool)
        .await;

        match qr {
            Ok(r) => Ok(r.user_id),
            Err(e) => Err(e.into()),
        }
    }

    /// Remove all tokens associated with the provided `user_id`.
    pub async fn user_clear_tokens_by_id(&self, user_id: &u64) -> DBResult<()> {
        let qr = sqlx::query!("DELETE FROM UserToken WHERE user_id = ?", user_id)
            .execute(&self.conn_pool)
            .await;

        println!("user_clear_tokens_by_id: {:?}", qr);

        match qr {
            Ok(_)  => Ok(()),
            Err(e) => Err(e.into()),
        }
    }

    pub async fn user_update_password_hash(&self, user_id: &u64, password_hash: String) -> DBResult<()> {
        let qr = sqlx::query!(
            "UPDATE User
            SET password_hash = ?
            WHERE id = ?",
            password_hash,
            user_id)
            .execute(&self.conn_pool)
            .await;

        match qr {
            Ok(r) if r.rows_affected() == 1 => Ok(()),
            Ok(_)  => Err(DatabaseServiceError::NoResult),
            Err(e) => Err(e.into()),
        }
    }

    /*  Chat room management  */

    /// Create a new chat room with the specified `room_name` returning the
    /// rooms `id` on success.
    pub async fn chat_room_create(&self, room_name: String) -> DBResult<u64> {
        let qr = sqlx::query!(
            "INSERT INTO Room (Name) VALUES (?);",
            room_name)
            .execute(&self.conn_pool)
            .await;

        match qr {
            Ok(r) if r.rows_affected() == 1 => Ok(r.last_insert_id()),
            Ok(_)  => Err(DatabaseServiceError::NoResult),
            Err(e) => Err(e.into()),
        }
    }

    pub async fn chat_room_change_name(&self, room_id: &u64, name: String) -> DBResult<()> {
        let qr = sqlx::query!(
            "UPDATE Room
            SET name = ?
            WHERE id = ?",
            name,
            room_id)
            .execute(&self.conn_pool)
            .await;

        match qr {
            Ok(r) if r.rows_affected() > 0 => Ok(()),
            Ok(_)  => Err(DatabaseServiceError::NoResult),
            Err(e) => Err(e.into()),
        }
    }

    /// Add the user specified by `user_id` to the chat room specified by
    /// `room_id`.
    pub async fn chat_room_add_user(&self, room_id: &u64, user_id: &u64) -> DBResult<()> {
        let qr = sqlx::query!(
            "INSERT INTO RoomMember VALUES (?, ?);",
            room_id,
            user_id)
            .execute(&self.conn_pool)
            .await;

        match qr {
            Ok(r) if r.rows_affected() == 1 => Ok(()),
            Ok(_)  => Err(DatabaseServiceError::NoResult),
            Err(e) => Err(e.into()),
        }
    }

    /// Remove the user specified by `user_id` from the chat room specified
    /// by `room_id`
    pub async fn chat_room_remove_user(&self, room_id: &u64, user_id: &u64) -> DBResult<()> {
        let qr = sqlx::query!(
            "DELETE FROM RoomMember
            WHERE room_id = ?
            AND user_id = ?",
            room_id,
            user_id)
            .execute(&self.conn_pool)
            .await;

        match qr {
            Ok(r) if r.rows_affected() == 1 => Ok(()),
            Ok(_)  => Err(DatabaseServiceError::NoResult),
            Err(e) => Err(e.into())
        }
    }

    /// Retrieve a list of user_ids in the chat room specified by `room_id`.
    /// The returned user_ids are sorted.
    pub async fn chat_room_get_user_ids(&self, room_id: &u64) -> DBResult<Vec<u64>> {
        let qr = sqlx::query!(
            "SELECT user_id
            FROM RoomMember
            WHERE room_id = ?
            ORDER BY user_id ASC",
            room_id)
            .fetch_all(&self.conn_pool)
            .await;

        match qr {
            Ok(r) if !r.is_empty() => Ok(Vec::from_iter(r.iter().map(|row| row.user_id))),
            Ok(_)  => Err(DatabaseServiceError::NoResult),
            Err(e) => Err(e.into()),
        }
    }

    /// Retrieve a list of usernames of members in the chat room specified by
    /// `room_id`. The returned usernames are sorted.
    pub async fn chat_room_get_usernames(&self, room_id: &u64) -> DBResult<Vec<String>> {
        let qr = sqlx::query!(
            "SELECT username
            FROM User
            WHERE id IN (
                SELECT user_id
                FROM RoomMember
                WHERE room_id = ?
            )
            ORDER BY username ASC",
            room_id)
            .fetch_all(&self.conn_pool)
            .await;
            
        match qr {
            Ok(r) => Ok(Vec::from_iter(r.iter().map(|row| row.username.clone()))),
            Err(e) => Err(e.into()),
        }
    }

}
