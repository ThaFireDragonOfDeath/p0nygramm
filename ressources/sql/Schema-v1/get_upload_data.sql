-- Input: upload_id
-- Returns: upload_filename; upload_timestamp; is_nsfw, uploader_id, uploader_username, upvotes
SELECT up.upload_filename, up.upload_timestamp, up.upload_is_nsfw, up.uploader, us.user_name, SUM(vo.vote_number) AS upvotes
FROM uploads up
INNER JOIN users us ON up.uploader = us.user_id
INNER JOIN votes_uploads vo ON vo.vote_upload = up.upload_id
WHERE up.uploader = $1::INT4
GROUP BY up.upload_filename, up.upload_timestamp, up.upload_is_nsfw, up.uploader, us.user_name;