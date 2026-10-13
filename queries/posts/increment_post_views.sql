UPDATE posts
SET
    views = views + 1
WHERE id = $1;