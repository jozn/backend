--  Drop table

DROP TABLE flip_tg.tg_username;

CREATE TABLE flip_tg.tg_username (
    username varchar(100) NOT NULL PRIMARY KEY,
    channel_id INT NOT NULL,
    user_id INT NULL,
    not_occupied BOOL NULL
)
ENGINE=InnoDB
DEFAULT CHARSET=utf8mb4
COLLATE=utf8mb4_general_ci;


CREATE TABLE flip_tg.tg_channel (
    channel_id INT NOT NULL PRIMARY KEY,
    username varchar(100),
    data BLOB
)
ENGINE=InnoDB
DEFAULT CHARSET=utf8mb4
COLLATE=utf8mb4_general_ci;

CREATE TABLE flip_tg.tg_channel_msg (
    channel_id INT NOT NULL PRIMARY KEY,
    message_id INT NOT NULL,
    seq_id INT,
    flip_gid BIGINT UNSIGNED,
    deleted BOOL,
    flip_sync BOOL,
    sync_time INT,
    data BLOB
)
ENGINE=InnoDB
DEFAULT CHARSET=utf8mb4
COLLATE=utf8mb4_general_ci;

CREATE TABLE flip_tg.tg_link (
    hash varchar(150) NOT NULL PRIMARY KEY,
    channel_id INT,
    expired BOOL
)
ENGINE=InnoDB
DEFAULT CHARSET=utf8mb4
COLLATE=utf8mb4_general_ci;
