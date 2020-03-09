-- Input: upload_id
-- Returns: comment timestamp; comment content; poster (id) and poster (username) unordered
SELECT c.comment_timestamp, c.comment_text, c.comment_poster, u.user_name
FROM uploads up
INNER JOIN tag_upload_map tum ON up.upload_id = tum.upload_id
INNER JOIN tags ta ON tum.tag_id = ta.tag_id
WHERE tum.upload_id = $1::INT4
ORDER BY c.comment_timestamp ASC;