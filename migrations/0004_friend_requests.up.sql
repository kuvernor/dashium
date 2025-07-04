CREATE TABLE friend_requests (
    friend_request_id SERIAL PRIMARY KEY,
    user_id INT NOT NULL,
    target_id INT NOT NULL,
    body TEXT NOT NULL,
    is_new SMALLINT NOT NULL DEFAULT 1,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,

    FOREIGN KEY (user_id) REFERENCES users (user_id),
    FOREIGN KEY (target_id) REFERENCES users (user_id)
);