DROP DATABASE twitter;
CREATE DATABASE IF NOT EXISTS twitter;
CREATE TABLE twitter.tweet (
    tweet_id BIGINT UNSIGNED auto_increment PRIMARY KEY,
    created_time BIGINT NULL DEFAULT 0,
    text_body TEXT NULL
) ENGINE=InnoDB
DEFAULT CHARSET=latin1
COLLATE=latin1_swedish_ci;


CREATE TABLE twitter.tweet_media (
     media_gid BIGINT NOT NULL PRIMARY KEY,
     media_type TINYINT NOT NULL,
     file_gid BIGINT NOT NULL
) ENGINE=InnoDB
DEFAULT CHARSET=latin1
COLLATE=latin1_swedish_ci;
CREATE UNIQUE INDEX tweet_media_by_type_IDX USING BTREE ON twitter.tweet_media (media_type,file_gid);


