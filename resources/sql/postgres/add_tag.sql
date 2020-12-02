-- Input: tag poster userid, tag id, upload id
-- Returns: None
INSERT INTO tag_upload_map (tag_poster, tag_id, upload_id)
VALUES ($1::INT4, $2::INT4, $3::INT4);