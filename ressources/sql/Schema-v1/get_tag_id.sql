-- Input: tag_text
-- Returns: tag_id
SELECT tag_id
FROM tags
WHERE tag_text = $1::VARCHAR;