-- Input: comment poster userid, upload id, comment content
-- Returns: None
INSERT INTO comments (comment_poster, comment_upload, comment_text)
VALUES ($1::INT4, $2::INT4, $3::TEXT);