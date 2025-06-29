CREATE TABLE messages (
    message_id SERIAL PRIMARY KEY,
    user_id INT NOT NULL,
    subject TEXT NOT NULL,
    body TEXT NOT NULL,
    username VARCHAR(50) NOT NULL,
    is_read SMALLINT NOT NULL DEFAULT 0,
    is_sender SMALLINT NOT NULL DEFAULT 1,
    target_id INT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,

    FOREIGN KEY (user_id) REFERENCES users (user_id),
    FOREIGN KEY (target_id) REFERENCES users (user_id)
);