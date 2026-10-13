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
where slug = $1
AND status = 'published';