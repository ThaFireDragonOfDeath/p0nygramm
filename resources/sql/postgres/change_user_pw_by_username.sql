-- Input: New hashed user password, username
UPDATE users
SET user_pass = $1::VARCHAR
WHERE user_name = $2::VARCHAR;