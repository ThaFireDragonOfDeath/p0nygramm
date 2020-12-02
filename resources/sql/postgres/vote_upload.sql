-- Input: upload_id, user_id (voter), vote_value
BEGIN TRANSACTION;

INSERT INTO votes_uploads (vote_upload, vote_user, vote_number)
VALUES ($1::INT4, $2::INT4, $3::INT4)
ON CONFLICT (vote_upload, vote_user)
DO UPDATE
SET vote_number = EXCLUDED.vote_number;

UPDATE uploads
SET upload_upvotes = upload_upvotes + $3::INT4
WHERE upload_id = $1::INT4;

COMMIT TRANSACTION;