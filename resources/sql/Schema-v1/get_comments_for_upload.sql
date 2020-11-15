-- Input: upload_id
-- Returns: comment timestamp; comment content; poster (id), poster (username) and upvotes ordered by date/time
SELECT c.comment_timestamp, c.comment_text, c.comment_poster, u.user_name, c.comment_upvotes
FROM comments c
INNER JOIN users u ON c.comment_poster = u.user_id
WHERE c.comment_upload = $1::INT4
ORDER BY c.comment_timestamp ASC;