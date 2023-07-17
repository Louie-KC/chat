use chrono::{DateTime, Utc};
use sqlx::{MySql, Pool};
use sqlx::mysql::MySqlPoolOptions;
use uuid::Uuid;
use crate::models::message::MessageResponse;

pub struct Database {
    db_pool: Pool<MySql>,
}

impl Database {
    pub async fn new(url: &str) -> Database {
        let pool = match MySqlPoolOptions::new().connect(&url).await {
            Ok(pool) => pool,
            Err(_) => panic!("Could not connect to database")
        };
        Database { db_pool: pool }
    }

    /// Create or register an account in the database. Generates and assigns a
    /// unique user ID for the new account. Returns Ok on success, and Error if
    /// the provided username is already taken.
    /// 
    /// ## Arguments
    /// * 'uname' - The users desired username
    /// * 'pword' - The users desired password
    pub async fn create_account(&self, uname: &str, pword: &str) -> Result<(), ()> {
        // Check if username is free
        let username_query = sqlx::query!(
            r"SELECT * FROM Account
            WHERE username = ?
            LIMIT 1", &uname)
            .fetch_one(&self.db_pool)
            .await;
        if let Ok(_) = username_query {
            return Err(())  // username not free
        }

        // Register
        let user_id = uuid::Uuid::new_v4().to_string();
        let query_result = sqlx::query(
            r"INSERT INTO Account (id, username, password)
            VALUES (?, ?, ?);")
            .bind(user_id)
            .bind(uname)
            .bind(pword)
            .execute(&self.db_pool)
            .await;

        match query_result {
            Ok(_)  => Ok(()),
            Err(_) => Err(())
        }
    }

    /// Attempts to login a user with the provided username and password.
    /// Generates a login token and inserts/updates the token into the database
    /// with an expiry time 12 hours from login time. Returns the token
    /// upon success. Returns and error on invalid login details.
    /// 
    /// ## Arguments
    /// * 'uname' - Username to use for login
    /// * 'pword' - Password to use for login
    pub async fn login(&self, uname: &str, pword: &str) -> Result<String, ()> {
        let query_result: Result<AccountLogin, _> = sqlx::query_as!(
            AccountLogin,
            r"SELECT id FROM Account
            WHERE username = ?
            AND password = ?", uname, pword)
            .fetch_one(&self.db_pool)
            .await;
        // println!("Login detail query");
        let id = match query_result {
            Ok(qr) => qr.id,
            Err(_) => return Err(())
        };
        println!("Correct login details");
        let token = Uuid::new_v4().to_string();
        let token_expiry = chrono::Utc::now() + chrono::Duration::hours(12);
        // Insert or update active token for the now logged in user
        let token_set = sqlx::query(
            "INSERT INTO ActiveToken (account_id, token, expiration)
            VALUES (?, ?, ?)
            ON DUPLICATE KEY UPDATE
            token = ?,
            expiration = ?;")
            .bind(&id)
            .bind(&token)
            .bind(&token_expiry)
            .bind(&token)
            .bind(&token_expiry)
            .execute(&self.db_pool)
            .await;

        println!("{:?}", token_set);
        match token_set {
            Ok(_)  => Ok(token),
            Err(_) => Err(())
        }
    }

    /// Converts a login token and returns the tokens assigned user id.
    /// An error is returned if the provided token is invalid (e.g. not mapped
    /// to a user id or expired).
    /// 
    /// ## Arguments
    /// * `token` -> The token to be converted to user/account id
    pub async fn token_to_uid(&self, token: &str) -> Result<String, ()> {
        let result:Result<ActiveToken, _> = sqlx::query_as!(
            ActiveToken,
            r"SELECT * FROM ActiveToken
            WHERE token = ?
            AND expiration > NOW()", token)
            .fetch_one(&self.db_pool)
            .await;

        match result {
            Ok(qr) => Ok(qr.account_id),
            Err(_) => Err(())
        }
    }

    /// Adds a message to the the specified chat with the provided contents.
    /// Returns Ok if successful, and an error if sending failed.
    /// 
    /// ## Arguments
    /// * `chat_id` - an identifier specifying the chat
    /// * `sender_id` - an identifier specifying the senders account
    /// * `content` - The content (text) of the message being sent.
    pub async fn add_message(
        &self,
        chat_id: &str,
        sender_id: &str,
        content: &str
    ) -> Result<(), ()> {
        let result = sqlx::query(
            "INSERT INTO Message (id, sender_id, chat_id, content, time_sent)
            VALUES (?, ?, ?, ?, ?)")
            .bind(Uuid::new_v4().to_string())
            .bind(&sender_id)
            .bind(&chat_id)
            .bind(&content)
            .bind(chrono::Utc::now())
            .execute(&self.db_pool)
            .await;

        // println!("{:?}", result);
        match result {
            Ok(_)  => Ok(()),
            Err(_) => Err(())
        }
    }

    /// Retrieves all messages for a user that were sent after a specified time.
    /// 
    /// ## Arguments
    /// * `requester_id` - The requesting users account ID
    /// * `from_time` - An ISO 8601 time
    pub async fn get_messages(
        &self,
        requester_id: &str,
        from_time: DateTime<Utc> 
    ) -> Result<Vec<MessageResponse>, ()> {
        let result: Result<Vec<MessageResponse>, _> = sqlx::query_as!(
            MessageResponse,
            r"SELECT id, sender_id, chat_id, content, time_sent FROM (
                SELECT *, ROW_NUMBER() OVER(
                    PARTITION BY chat_id
                    ORDER BY time_sent DESC
                ) row_num
                FROM Message
                WHERE chat_id IN (
                    SELECT DISTINCT chat_id FROM ChatParticipant
                    WHERE account_id = ?
                )
            ) sub
            WHERE row_num = 1
            AND time_sent > ?", requester_id, from_time)
            .fetch_all(&self.db_pool)
            .await;

        match result {
            Ok(mrs) => Ok(mrs),
            Err(_) => Err(())
        }
    }

    /// Retrieves messages from a conversation for a user that were sent after a
    /// specified time.
    /// 
    /// ## Arguments
    /// * `requester_id` - The requesting users account ID
    /// * `chat_id` - An identifier specifying the chat
    /// * `from_time` - An ISO 8601 time
    pub async fn get_conversation_messages(
        &self,
        requester_id: &str,
        chat_id: &str,
        from_time: DateTime<Utc>
    ) -> Vec<MessageResponse> {
        let result: Vec<MessageResponse> = sqlx::query_as!(
            MessageResponse,
            r"SELECT * FROM Message
            WHERE chat_id = ?
            AND ? IN (
                SELECT account_id FROM ChatParticipant
                WHERE chat_id = ?
            )
            AND time_sent >= ?
            ORDER BY time_sent DESC", &chat_id, requester_id, chat_id, from_time)
            .fetch_all(&self.db_pool)
            .await
            .unwrap();
        result
    }

}

struct AccountLogin {
    id: String
}

struct ActiveToken {
    account_id: String,
    token: String,
    expiration: chrono::DateTime<chrono::Utc>
}