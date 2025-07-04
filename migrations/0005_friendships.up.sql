CREATE TABLE friendships (
    friendship_id SERIAL PRIMARY KEY,
    user1 INT NOT NULL,
    user2 INT NOT NULL,
    is_new1 SMALLINT NOT NULL DEFAULT 1,
    is_new2 SMALLINT NOT NULL DEFAULT 1,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,

    FOREIGN KEY (user1) REFERENCES users (user_id),
    FOREIGN KEY (user2) REFERENCES users (user_id)
);