-- Input: username, hashed password, user is mod
-- Returns: user id
INSERT INTO public.users (user_name, user_pass, user_is_mod)
VALUES ($1::VARCHAR, $2::VARCHAR, $3::BOOL)
RETURNING user_id;