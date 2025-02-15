use log::warn;
use sqlx::{
    MySql,
    Pool
};
use sqlx::mysql::MySqlPoolOptions;
use uuid::Uuid;

use common::{
    ChatMessage,
    ChatRoom,
    UserInfo
};

use crate::models::{
    DBAuthInfo,
    DBRoomMember,
    DBUser
};

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
    /// provided `username`. This check if not case sensitive.
    pub async fn user_exists(&self, username: &str) -> DBResult<bool> {
        let qr = sqlx::query!(
            "SELECT COUNT(*) as count
            FROM User
            WHERE UPPER(username) = ?",
            username.to_ascii_uppercase())
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
    /// `username`. The underlying search is not case sensitive.
    pub async fn user_get_by_username(&self, username: &str) -> DBResult<DBUser> {
        let qr = sqlx::query_as!(
            DBUser,
            "SELECT *
            FROM User
            WHERE UPPER(username) = ?;",
            username.to_ascii_uppercase())
            .fetch_one(&self.conn_pool)
            .await;

        Ok(qr?)
    }

    /// Retrieve the User record from the connected database with the provided
    /// `user_id`.
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
    pub async fn user_set_token(&self, user_id: &u64, token: &Uuid, user_agent: &str) -> DBResult<()> {
        let qr = sqlx::query!(
            "INSERT INTO UserToken (token, user_id, user_agent) VALUES (?, ?, ?)
            ON DUPLICATE KEY UPDATE user_agent = ?;",
            token.to_string(),
            user_id,
            user_agent,
            user_agent)
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

    /// Remove the provided `token` from mapping to `user_id`, effectively
    /// logging out the user from their current client.
    pub async fn user_remove_token(&self, user_id: &u64, token: &Uuid) -> DBResult<()> {
        let qr = sqlx::query!("DELETE FROM UserToken WHERE user_id = ? AND token = ?",
            user_id,
            token.to_string())
            .execute(&self.conn_pool)
            .await;

        match qr {
            Ok(r) if r.rows_affected() > 0 => Ok(()),
            Ok(_)  => Err(DatabaseServiceError::NoResult),
            Err(e) => Err(e.into())
        }
    }

    /// Retrieve information about the associated auth tokens for a provided `user_id`.
    /// `token` is used to determine which of the returned tokens is a made by the
    /// requesting user.
    pub async fn user_get_associated_tokens(&self, user_id: &u64, token: &Uuid) -> DBResult<Vec<DBAuthInfo>> {
        let qr = sqlx::query_as!(
            DBAuthInfo,
            "SELECT user_agent, time_set, IF(token = ?, true, false) as 'is_requester: _'
            FROM UserToken
            WHERE user_id = ?
            ORDER BY time_set",
            token.to_string(),
            user_id)
            .fetch_all(&self.conn_pool)
            .await;

        match qr {
            Ok(info) => Ok(info),
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

    /// Get a list of chat rooms that the user specified by `user_id` are
    /// members of.
    pub async fn chat_room_list_for_user(&self, user_id: &u64) -> DBResult<Vec<ChatRoom>> {
        let qr = sqlx::query_as!(
            ChatRoom,
            "SELECT *
            FROM Room
            WHERE id IN (
                SELECT room_id
                FROM RoomMember
                WHERE user_id = ?
            );",
            user_id)
            .fetch_all(&self.conn_pool)
            .await;

        Ok(qr?)
    }

    /// Create a new chat room with the specified `room_name` returning the
    /// rooms `id` on success.
    pub async fn chat_room_create(&self, room_name: &str) -> DBResult<u64> {
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

    pub async fn chat_room_change_name(&self, room_id: &u64, name: &str) -> DBResult<()> {
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
            Ok(e)  => {
                // debug!("{:?}", e);
                Err(DatabaseServiceError::NoResult)
            },
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

    /// Retrieve a list of users that are members of a room specified by
    /// `room_id`. User info includes user IDs and usernames.
    pub async fn chat_room_get_users(&self, room_id: &u64) -> DBResult<Vec<DBRoomMember>> {
        let qr = sqlx::query_as!(
            DBRoomMember,
            "SELECT u.id AS 'user_id', u.username AS 'username'
            FROM User u
            INNER JOIN RoomMember rm ON u.id = rm.user_id
            WHERE rm.room_id = ?",
            room_id)
            .fetch_all(&self.conn_pool)
            .await;

        match qr {
            Ok(members) if !members.is_empty() => Ok(members),
            Ok(_)  => Err(DatabaseServiceError::NoResult),
            Err(e) => Err(e.into()),
        }
    }


    /*  Chat interaction */

    /// Retrieve a window of messages from a particular chat room specified by
    /// `room_id`. The messages within this window are ordered with having the
    /// oldest/earliest message first, and newest/latest message last.
    /// 
    /// `offset` controls how far away from the newest message the window starts.
    /// `limit` then controls the window size (I.O.W the quantity of messages).
    pub async fn chat_room_read_messages(&self, room_id: &u64, offset: &u64, limit: &u64) -> DBResult<Vec<ChatMessage>> {
        let qr = sqlx::query_as!(
            ChatMessage,
            "SELECT *
            FROM Message
            WHERE room_id = ?
            ORDER BY time_sent DESC
            LIMIT ?
            OFFSET ?;",
            room_id,
            limit,
            offset)
            .fetch_all(&self.conn_pool)
            .await;
        
        match qr {
            Ok(messages) => Ok(messages),
            Err(e) => Err(e.into()),
        }
    }

    /// Record a new message for a particular chat room.
    /// 
    /// The Option fields of the `message` are ignored.
    /// 
    /// `user_id` should be derived from the auth token, instead of the
    /// `sender_id` of the ChatMessage struct.
    pub async fn chat_room_send_message(&self, user_id: &u64, message: &ChatMessage) -> DBResult<()> {
        if message.id.is_some() || message.time_sent.is_some() {
            warn!("chat_room_send_message invoked with populated Option fields: {:?}", message);
        }

        let qr = sqlx::query!(
            "INSERT INTO Message (room_id, sender_id, body)
            VALUES (?, ?, ?)",
            message.room_id,
            user_id,
            message.body)
            .execute(&self.conn_pool)
            .await;

        match qr {
            Ok(r) if r.rows_affected() > 0 => Ok(()),
            Ok(_)  => Ok(()),
            Err(e) => Err(e.into()),
        }
    }

    /// Retrieve a list of users with `search_term` in their username.
    /// 
    /// Users that have blocked the user with the provided `user_id` are
    /// excluded from the result. This `user_id` is intended to be of the
    /// user making the search.
    pub async fn user_search_global(&self, user_id: &u64, search_term: &str) -> DBResult<Vec<UserInfo>> {
        let qr = sqlx::query_as!(
            UserInfo,
            "SELECT id, username
            FROM User
            WHERE username LIKE ?
            AND id NOT IN (
                SELECT user_id
                FROM UserAssociation
                WHERE other_user_id = ?
                AND association = 'BLOCK'
            )",
            format!("%{}%", search_term),
            user_id)
            .fetch_all(&self.conn_pool)
            .await;

        match qr {
            Ok(users) => Ok(users),
            Err(e) => Err(e.into()),
        }
    }

    /// Associate the users with `user_id` and `other_id` as friends in one
    /// direction.
    /// 
    /// `user_id --friends with--> other_id`.
    pub async fn user_association_set_friend(&self, user_id: &u64, other_id: &u64) -> DBResult<()> {
        let qr = sqlx::query!("
            INSERT INTO UserAssociation (user_id, other_user_id, association)
            VALUES (?, ?, 'FRIEND')
            ON DUPLICATE KEY UPDATE association = 'FRIEND'",
            user_id,
            other_id)
            .execute(&self.conn_pool)
            .await;

        match qr {
            Ok(r) if r.rows_affected() > 0 => Ok(()),
            Ok(_)  => Err(DatabaseServiceError::NoResult),
            Err(e) => Err(e.into()),
        }
    }

    /// Set the association of `user_id` to `other_id` as a blocked association.
    /// 
    /// `user_id --has blocked--> other_id`.
    pub async fn user_association_set_block(&self, user_id: &u64, other_id: &u64) -> DBResult<()> {
        let qr = sqlx::query!("
            INSERT INTO UserAssociation (user_id, other_user_id, association)
            VALUES (?, ?, 'BLOCK')
            ON DUPLICATE KEY UPDATE association = 'BLOCK'",
            user_id,
            other_id)
            .execute(&self.conn_pool)
            .await;

        match qr {
            Ok(r) if r.rows_affected() > 0 => Ok(()),
            Ok(_)  => Err(DatabaseServiceError::NoResult),
            Err(e) => Err(e.into()),
        }
    }

    /// Remove any association that exists from `user_id` to `other_id`.
    pub async fn user_association_delete(&self, user_id: &u64, other_id: &u64) -> DBResult<()> {
        let qr = sqlx::query!(
            "DELETE FROM UserAssociation
            WHERE user_id = ?
            AND other_user_id = ?",
            user_id,
            other_id)
            .execute(&self.conn_pool)
            .await;

        match qr {
            Ok(r) if r.rows_affected() > 0 => Ok(()),
            Ok(_)  => Err(DatabaseServiceError::NoResult),
            Err(e) => Err(e.into()),
        }
    }

    /// Get a list of users that are accepted friends of `user_id`.
    pub async fn user_association_get_friends(&self, user_id: &u64) -> DBResult<Vec<UserInfo>> {
        let qr = sqlx::query_as!(
            UserInfo,
            "SELECT id, username
            FROM User
            WHERE id IN (
                SELECT requester.other_user_id
                FROM UserAssociation requester
                INNER JOIN UserAssociation requestee
                ON requester.other_user_id = requestee.user_id
                WHERE requester.association = 'FRIEND'
                AND requester.user_id = ?
            )",
            user_id)
            .fetch_all(&self.conn_pool)
            .await;

        match qr {
            Ok(friends) => Ok(friends),
            Err(e) => Err(e.into()),
        }
    }

    /// Get a list of users that have sent `user_id` a (currently unaccepted)
    /// friend request.
    pub async fn user_association_get_friend_requesters(&self, user_id: &u64) -> DBResult<Vec<UserInfo>> {
        let qr = sqlx::query_as!(
            UserInfo,
            "SELECT id, username
            FROM User
            WHERE id IN (
                SELECT requester.user_id
                FROM UserAssociation requester
                LEFT JOIN UserAssociation requestee
                ON requester.user_id = requestee.other_user_id
                WHERE requester.association = 'FRIEND'
                AND requester.other_user_id = ?
                AND requestee.association IS NULL
            )",
            user_id)
            .fetch_all(&self.conn_pool)
            .await;

        match qr {
            Ok(requesting_users) => Ok(requesting_users),
            Err(e) => Err(e.into())
        }
    }

    /// Get a list of users that have been sent a friend request by `user_id`, but
    /// have not accepted.
    pub async fn user_association_get_unaccepted_friends(&self, user_id: &u64) -> DBResult<Vec<UserInfo>> {
        let qr = sqlx::query_as!(
            UserInfo,
            "SELECT id, username
            FROM User
            WHERE id IN (
                SELECT requester.other_user_id
                FROM UserAssociation requester
                LEFT JOIN UserAssociation requestee
                ON requester.other_user_id = requestee.user_id
                WHERE requester.association = 'FRIEND'
                AND requester.user_id = ?
                AND requestee.association IS NULL
            )",
            user_id)
            .fetch_all(&self.conn_pool)
            .await;

        match qr {
            Ok(awaiting_responses) => Ok(awaiting_responses),
            Err(e) => Err(e.into())
        }
    }

    /// Get a list of users that have been blocked by `user_id`.
    pub async fn user_association_get_blocked(&self, user_id: &u64) -> DBResult<Vec<UserInfo>> {
        let qr = sqlx::query_as!(
            UserInfo,
            "SELECT id, username
            FROM User
            WHERE id IN (
                SELECT other_user_id
                FROM UserAssociation
                WHERE user_id = ?
                AND association = 'BLOCK'
            )",
            user_id)
            .fetch_all(&self.conn_pool)
            .await;

        match qr {
            Ok(blocked) => Ok(blocked),
            Err(e) => Err(e.into()),
        }
    }

}
