-- Input: upload_id
-- Returns: tag_text; tag_upvotes; ordered by upvotes
SELECT ta.tag_text, tum.tag_upvotes
FROM uploads up
INNER JOIN tag_upload_map tum ON up.upload_id = tum.upload_id
INNER JOIN tags ta ON tum.tag_id = ta.tag_id
WHERE tum.upload_id = $1::INT4
ORDER BY tum.tag_upvotes DESC;