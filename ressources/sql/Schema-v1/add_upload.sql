-- Input: upload filename, upload is nsfw, uploader userid
-- Returns: upload_id
INSERT INTO uploads (upload_filename, upload_is_nsfw, uploader)
VALUES ($1::VARCHAR, $2::BOOL, $3::INT4)
RETURNING upload_id;