SELECT 
    id,
    title,
    seo_title,
    meta_description,
    slug,
    content,
    status,
    views,
    published_at,
    created_at,
    updated_at
FROM posts
WHERE status = $1
ORDER BY published_at DESC NULLS LAST, created_at DESC;