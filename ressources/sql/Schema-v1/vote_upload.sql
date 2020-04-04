-- Input: upload_id, user_id (voter), vote_value
INSERT INTO votes (vote_upload, vote_user, vote_number)
VALUES ($1::INT4, $2::INT4, $3::INT4)
ON CONFLICT (vote_upload, vote_user)
DO UPDATE
SET vote_number = EXCLUDED.vote_number;