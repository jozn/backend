CREATE TABLE twitter.tweet (
    tweet_id BIGINT UNSIGNED auto_increment PRIMARY KEY,
    created_time BIGINT NULL DEFAULT 0,
    text_body TEXT NULL
) ENGINE=InnoDB
DEFAULT CHARSET=latin1
COLLATE=latin1_swedish_ci;
