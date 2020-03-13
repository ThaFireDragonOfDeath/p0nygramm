-- Input: tag_text
-- Returns: tag_id or nothing (if tag_text is already saved)
INSERT INTO tags (tag_text)
VALUES ($1::TEXT)
ON CONFLICT (tag_text) DO NOTHING
RETURNING tag_id;