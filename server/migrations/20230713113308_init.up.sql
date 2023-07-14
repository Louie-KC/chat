-- Add up migration script here
CREATE TABLE IF NOT EXISTS Account (
    id CHAR(37) NOT NULL,
    username VARCHAR(128) NOT NULL,
    password VARCHAR(128) NOT NULL,
    PRIMARY KEY(id)
);

CREATE TABLE IF NOT EXISTS Chat (
    id CHAR(37) NOT NULL,
    name TINYTEXT NOT NULL,
    PRIMARY KEY (id)
);

CREATE TABLE IF NOT EXISTS ChatParticipant (
    chat_id CHAR(37) NOT NULL,
    account_id CHAR(37) NOT NULL,
    FOREIGN KEY (chat_id) REFERENCES Chat(id),
    FOREIGN KEY (account_id) REFERENCES Account(id)
);

CREATE TABLE IF NOT EXISTS Message (
    id CHAR(37) NOT NULL,
    sender_id CHAR(37) NOT NULL,
    chat_id CHAR(37) NOT NULL,
    content TEXT NOT NULL,
    -- time_sent DATETIME(3), -- 3 gives millisecond
    PRIMARY KEY (id),
    FOREIGN KEY (sender_id) REFERENCES Account(id),
    FOREIGN KEY (chat_id) REFERENCES Chat(id)
);

CREATE TABLE IF NOT EXISTS ActiveToken (
    token CHAR(37) NOT NULL,
    account_id CHAR(37) NOT NULL,
    expiration TIMESTAMP NOT NULL,
    PRIMARY KEY (account_id),
    FOREIGN KEY (account_id) REFERENCES Account(id)
);


DROP PROCEDURE IF EXISTS delete_expired_tokens;

CREATE PROCEDURE delete_expired_tokens()
BEGIN
    DELETE FROM ActiveToken WHERE expiration <= NOW();
END ;

DROP EVENT IF EXISTS minutely_token_cleanup;

CREATE EVENT minutely_token_cleanup
    ON SCHEDULE EVERY 1 MINUTE
    DO
        CALL delete_expired_tokens();