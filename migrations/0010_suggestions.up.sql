CREATE TABLE suggestions (
    suggestion_id SERIAL PRIMARY KEY,
    user_id INT NOT NULL,
    level_id INT NOT NULL,
    feature	VARCHAR(10) NOT NULL,
    stars SMALLINT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,

    FOREIGN KEY (user_id) REFERENCES users (user_id),
    FOREIGN KEY (level_id) REFERENCES levels (level_id) ON DELETE CASCADE
);