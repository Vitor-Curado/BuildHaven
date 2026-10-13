SELECT
    p.id,
    p.title,
    p.seo_title,
    p.meta_description,
    p.slug,
    p.content,
    p.status,
    p.views,
    p.published_at,
    p.created_at,
    p.updated_at
FROM posts AS p
INNER JOIN posts_tags AS pt ON pt.post_id = p.id
INNER JOIN tags AS t ON t.id = pt.tag_id
WHERE t.slug = $1
    AND p.status = 'published'
ORDER BY p.published_at DESC NULLS LAST, p.created_at DESC;