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
    PRIMARY KEY (id),
    FOREIGN KEY (sender_id) REFERENCES Account(id),
    FOREIGN KEY (chat_id) REFERENCES Chat(id)
);

CREATE TABLE IF NOT EXISTS ActiveToken (
    token CHAR(37) NOT NULL,
    account_id CHAR(37) NOT NULL,
    PRIMARY KEY (token),
    FOREIGN KEY (account_id) REFERENCES Account(id)
);

-- INSERT INTO Account (id, username, password)
-- VALUES ("0000000000000000000000000000000000000", "devtest", "12345");

-- INSERT INTO Account (id, username, password)
-- VALUES ("1234567890123456789012345678901234567", "test_user", "password");

-- INSERT INTO Chat (id, name)
-- VALUES ("1111111111111111111111111111111111111", "test_chat");

-- INSERT INTO ChatParticipant (chat_id, account_id)
-- VALUES ("1111111111111111111111111111111111111", "0000000000000000000000000000000000000");

-- INSERT INTO ChatParticipant (chat_id, account_id)
-- VALUES ("1111111111111111111111111111111111111", "1234567890123456789012345678901234567");