-- Input: comment_id, user_id (voter), vote_value
BEGIN TRANSACTION;

INSERT INTO votes_comments (vote_comment , vote_user, vote_number)
VALUES ($1::INT4, $2::INT4, $3::INT4)
ON CONFLICT (vote_comment, vote_user)
DO UPDATE
SET vote_number = EXCLUDED.vote_number;

UPDATE comments
SET comment_upvotes = comment_upvotes + $3::INT4
WHERE comment_id = $1::INT4;

COMMIT TRANSACTION;