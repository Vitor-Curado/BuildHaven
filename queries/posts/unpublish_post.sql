UPDATE posts
SET 
    status = 'draft',
    published_at = NULL,
    updated_at = NOW()
WHERE id = $1;