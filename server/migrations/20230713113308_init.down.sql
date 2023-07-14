-- Add down migration script here
DROP EVENT IF EXISTS minutely_token_cleanup;
DROP PROCEDURE IF EXISTS delete_expired_tokens;

DROP TABLE IF EXISTS ActiveToken;
DROP TABLE IF EXISTS Message;
DROP TABLE IF EXISTS ChatParticipant;
DROP TABLE IF EXISTS Chat;
DROP TABLE IF EXISTS Account;