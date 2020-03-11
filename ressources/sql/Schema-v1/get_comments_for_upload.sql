-- Input: upload_id
-- Returns: comment timestamp; comment content; poster (id), poster (username) and upvotes ordered by date/time
SELECT c.comment_timestamp, c.comment_text, c.comment_poster, u.user_name, SUM(v.vote_number) AS upvotes
FROM comments c
INNER JOIN users u ON c.comment_poster = u.user_id
INNER JOIN votes v ON c.comment_id = v.vote_comment
WHERE c.comment_upload = $1::INT4
GROUP BY c.comment_timestamp, c.comment_text, c.comment_poster, u.user_name
ORDER BY c.comment_timestamp ASC;