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
ORDER BY created_at DESC
LIMIT $1 OFFSET $2;