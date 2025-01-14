-- Add up migration script here
CREATE TABLE User (
    id BIGINT UNSIGNED NOT NULL AUTO_INCREMENT,
    username VARCHAR(128) NOT NULL,
    password_hash VARCHAR(256) NOT NULL,
    PRIMARY KEY (id),
    UNIQUE (username)
);

CREATE TABLE UserToken (
    token CHAR(36),
    user_id BIGINT UNSIGNED NOT NULL,
    time_set DATETIME DEFAULT NOW(),
    PRIMARY KEY (token),
    FOREIGN KEY (user_id) REFERENCES User(id)
);

CREATE TABLE UserAssociation (
    user_id BIGINT UNSIGNED NOT NULL,
    other_user_id BIGINT UNSIGNED NOT NULL,
    association ENUM("FRIEND", "BLOCK"),
    PRIMARY KEY (user_id, other_user_id),
    FOREIGN KEY (user_id) REFERENCES User(id),
    FOREIGN KEY (other_user_id) REFERENCES User(id)
);

CREATE TABLE Room (
    id BIGINT UNSIGNED NOT NULL AUTO_INCREMENT,
    name VARCHAR(128) NOT NULL,
    PRIMARY KEY (id)
);

CREATE TABLE RoomMember (
    room_id BIGINT UNSIGNED NOT NULL,
    user_id BIGINT UNSIGNED NOT NULL,
    PRIMARY KEY (room_id, user_id),
    FOREIGN KEY (room_id) REFERENCES Room(id),
    FOREIGN KEY (user_id) REFERENCES User(id)
);

CREATE TABLE Message (
    id BIGINT UNSIGNED AUTO_INCREMENT,
    room_id BIGINT UNSIGNED NOT NULL,
    sender_id BIGINT UNSIGNED NOT NULL,
    body VARCHAR(1000) NOT NULL,
    -- TIMESTAMP is UTC, DATETIME is naive
    time_sent TIMESTAMP NOT NULL DEFAULT NOW(),
    PRIMARY KEY(id, room_id),
    FOREIGN KEY (room_id) REFERENCES Room(id),
    FOREIGN KEY (sender_id) REFERENCES User(id)
);