CREATE TABLE level_reports (
    level_report_id SERIAL PRIMARY KEY,
    level_id INT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,

    FOREIGN KEY (level_id) REFERENCES levels (level_id) ON DELETE CASCADE
);