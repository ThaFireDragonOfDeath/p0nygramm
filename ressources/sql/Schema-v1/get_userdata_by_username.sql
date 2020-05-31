-- Input: username
-- Returns: user_id, username, hashed password, is mod
SELECT us.user_id, us.user_name, us.user_pass, us.user_is_mod
FROM users us
WHERE LOWER(us.user_name) = LOWER($1::VARCHAR);