SELECT id, name, slug, created_at, colour
FROM tags
WHERE slug = $1;