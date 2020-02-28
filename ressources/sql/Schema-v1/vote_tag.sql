-- Input: upload_id, tag_id, user_id, vote_value TODO
INSERT INTO votes (vote_tagmap, vote_user, vote_number)
VALUES ($1::INT4, $2::INT4, $3::INT4)
ON CONFLICT (vote_tagmap, vote_user)
DO UPDATE
SET vote_number = EXCLUDED.vote_number;