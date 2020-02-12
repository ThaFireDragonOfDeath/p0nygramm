-- Input: upload_id
-- Returns: comment timestamp; comment content; upvotes; poster (id) and poster (username)
SELECT c.comment_timestamp, c.comment_text, c.comment_upvotes, c.comment_poster, u.user_name
FROM comments c
INNER JOIN users u ON c.comment_poster = u.user_id
WHERE c.comment_upload = $1::INT4;