-- Input: upload_id
-- Returns: upload_filename; upload_timestamp; is_nsfw, uploader_id, uploader_username, upvotes
SELECT up.upload_filename, up.upload_timestamp, up.upload_is_nsfw, up.upload_type, up.uploader, us.user_name, up.upload_upvotes
FROM uploads up
INNER JOIN users us ON up.uploader = us.user_id
WHERE up.uploader = $1::INT4;