CREATE TABLE levels (
    level_id SERIAL PRIMARY KEY,
    level_name TEXT NOT NULL,
    description TEXT NOT NULL,
    level_string TEXT NOT NULL,
    extra_string TEXT NOT NULL,
    level_info TEXT NOT NULL,
    username TEXT NOT NULL,
    user_id INT NOT NULL,
    
    version INT NOT NULL,
    length SMALLINT NOT NULL DEFAULT 0,
    official_song SMALLINT NOT NULL,
    
    original SMALLINT NOT NULL DEFAULT 0,
    
    unlisted SMALLINT NOT NULL DEFAULT 0,

    likes INT NOT NULL DEFAULT 0,
    dislikes INT NOT NULL DEFAULT 0,
    downloads INT NOT NULL DEFAULT 0,
    objects INT NOT NULL DEFAULT 0,
    coins SMALLINT NOT NULL DEFAULT 0,
    password TEXT NOT NULL,
    is_two_player SMALLINT NOT NULL DEFAULT 0,
    song_id INT NOT NULL DEFAULT 0,
    song_ids TEXT NOT NULL DEFAULT '',
    sfx_ids TEXT NOT NULL DEFAULT '',
    
    game_version SMALLINT NOT NULL DEFAULT 22,
    binary_version SMALLINT NOT NULL DEFAULT 45,
    
    requested_stars SMALLINT NOT NULL DEFAULT 0,
    is_auto SMALLINT NOT NULL DEFAULT 0,
    is_ldm SMALLINT NOT NULL DEFAULT 0,
    is_rated SMALLINT NOT NULL DEFAULT 0,
    difficulty SMALLINT NOT NULL DEFAULT 0,
    demon_difficulty SMALLINT NOT NULL DEFAULT 0,
    is_demon SMALLINT NOT NULL DEFAULT 0,
    stars SMALLINT NOT NULL DEFAULT 0,
    feature_score INT NOT NULL DEFAULT 0,
    verified_coins SMALLINT NOT NULL DEFAULT 0,
    wt INT NOT NULL DEFAULT 0,
    wt2 INT NOT NULL DEFAULT 0,
    daily_number SMALLINT NOT NULL DEFAULT 0,
    epic SMALLINT NOT NULL DEFAULT 0,
    is_gauntlet SMALLINT NOT NULL DEFAULT 0,
    verification_time INT NOT NULL DEFAULT 0,

    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,

    FOREIGN KEY (user_id) REFERENCES users (user_id) ON DELETE CASCADE
);
