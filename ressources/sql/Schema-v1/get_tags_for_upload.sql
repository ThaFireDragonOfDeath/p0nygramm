-- Input: upload_id
-- Returns: tag_text; tag_upvotes; ordered by upvotes
SELECT ta.tag_text, SUM(vo.vote_number) AS upvotes
FROM uploads up
INNER JOIN tag_upload_map tum ON up.upload_id = tum.upload_id
INNER JOIN tags ta ON tum.tag_id = ta.tag_id
INNER JOIN votes_tum vo ON tum.tag_id = vo.vote_tagmap
WHERE tum.upload_id = $1::INT4
GROUP BY ta.tag_text
ORDER BY upvotes DESC;