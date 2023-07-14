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

    pub async fn create_account(&self, uname: &str, pword: &str) -> Result<(), ()> {
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

    pub async fn add_message(&self, chat_id: &str, sender_id: &str, content: &str) -> Result<(), ()> {
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

    pub async fn get_messages(
        &self,
        requester_id: &str,
        from: DateTime<Utc>
    ) -> Result<Vec<MessageResponse>, ()> {
        let result: Result<Vec<MessageResponse>, _> = sqlx::query_as!(
            MessageResponse,
            r"SELECT * FROM Message
            WHERE chat_id IN (
                SELECT DISTINCT chat_id FROM ChatParticipant
                WHERE account_id = ?
            )
            AND time_sent >= ?
            ORDER BY time_sent DESC", requester_id, from)
            .fetch_all(&self.db_pool)
            .await;

        match result {
            Ok(mrs) => Ok(mrs),
            Err(_) => Err(())
        }
    }

    pub async fn get_conversation_messages(
        &self,
        requester_id: &str,
        chat_id: &str,
        from: DateTime<Utc>
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
            ORDER BY time_sent DESC", &chat_id, requester_id, chat_id, from)
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