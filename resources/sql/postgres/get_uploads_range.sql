-- Input: upload_id (start id), upload_id (end id), allow sfw, allow nsfw
-- Returns: upload_id, upload_filename, upload_is_nsfw, upload_type
SELECT u.upload_id, u.upload_filename, u.upload_is_nsfw, u.upload_type
FROM uploads u
WHERE (u.upload_id <= $1::INT4) AND (u.upload_id >= $2::INT4)
AND ( ($3::BOOL = true AND u.upload_is_sfw = $3::BOOL) OR ($4::BOOL = true AND u.upload_is_nsfw = $4::BOOL) )
ORDER BY u.upload_id DESC
LIMIT 5000;