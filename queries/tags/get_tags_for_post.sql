SELECT t.id, t.name, t.slug, t.created_at, t.colour
FROM tags AS t
INNER JOIN posts_tags AS pt ON pt.tag_id = t.id
WHERE pt.post_id = $1
ORDER BY t.name ASC;