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
WHERE id = $1;