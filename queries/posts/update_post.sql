UPDATE posts
SET
    title = $2,
    seo_title = $3,
    meta_description = $4,
    content = $5,
    updated_at = NOW()
WHERE id = $1;