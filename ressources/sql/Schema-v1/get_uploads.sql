-- Input: upload_id (start id), max_count (how many entries will be returned), allow sfw, allow nsfw
-- Returns: upload_id, upload_filename, upload_is_nsfw
SELECT u.upload_id, u.upload_filename, u.upload_is_nsfw
FROM uploads u
WHERE (u.upload_id <= $1::INT4)
AND ( ($3::BOOL = true AND u.upload_is_sfw = $3::BOOL) OR ($4::BOOL = true AND u.upload_is_nsfw = $4::BOOL) )
ORDER BY u.upload_id DESC
LIMIT $2::INT2;