-- Input: upload_id, tag_id, user_id (voter), vote_value
BEGIN TRANSACTION;

INSERT INTO votes_tum (vote_tagmap, vote_user, vote_number)
SELECT tum.tum_id, $3::INT4, $4::INT4
FROM tag_upload_map AS tum
INNER JOIN votes_tum AS v_tum ON tum.tum_id = vote_tagmap
WHERE tum.tag_id = $2::INT4 AND tum.upload_id = $1::INT4
ON CONFLICT (vote_tagmap, vote_user)
DO UPDATE
SET vote_number = EXCLUDED.vote_number;

UPDATE tag_upload_map
SET tag_upvotes = tag_upvotes + $3::INT4
WHERE upload_id = $1::INT4 AND tag_id = $2::INT4;

COMMIT TRANSACTION;