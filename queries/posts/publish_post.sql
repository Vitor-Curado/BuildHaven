UPDATE posts
SET
    status = 'published',
    published_at = NOW(),
    updated_at = NOW()
WHERE id = $1;