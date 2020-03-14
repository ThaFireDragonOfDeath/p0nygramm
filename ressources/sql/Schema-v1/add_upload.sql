-- Input: upload filename, upload is sfw, upload is nsfw, uploader userid
-- Returns: upload_id
INSERT INTO uploads (upload_filename, upload_is_sfw, upload_is_nsfw, uploader)
VALUES ($1::VARCHAR, $2::BOOL, $3::BOOL, $4::INT4)
RETURNING upload_id;