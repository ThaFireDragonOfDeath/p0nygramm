-- Input: New hashed user password, user id
UPDATE users
SET user_pass = $1::VARCHAR
WHERE user_id = $2::INT4;