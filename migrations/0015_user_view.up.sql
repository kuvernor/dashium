CREATE VIEW user_view AS
SELECT *, ROW_NUMBER () OVER (
    ORDER BY stars DESC
) AS rank
FROM users;