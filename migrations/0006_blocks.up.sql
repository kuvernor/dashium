CREATE TABLE blocks (
    block_id SERIAL PRIMARY KEY,
    blocker_id INT NOT NULL,
    blocked_id INT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,

    FOREIGN KEY (blocker_id) REFERENCES users (user_id),
    FOREIGN KEY (blocked_id) REFERENCES users (user_id)
);
