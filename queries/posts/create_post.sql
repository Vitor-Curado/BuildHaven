INSERT INTO posts (id, title, seo_title, meta_description, slug, content)
    VALUES ($1, $2, $3, $4, $5, $6)
    RETURNING 
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
        updated_at;