-- Input: upload filename, upload is sfw, upload is nsfw, upload type, uploader userid
-- Returns: upload_id
INSERT INTO uploads (upload_filename, upload_is_sfw, upload_is_nsfw, upload_type, uploader)
VALUES ($1::VARCHAR, $2::BOOL, $3::BOOL, $4::UploadType, $5::INT4)
RETURNING upload_id;