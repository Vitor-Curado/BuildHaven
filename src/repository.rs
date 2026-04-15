use crate::models::{Food, NewPost, Post};
use sqlx::PgPool;
use uuid::Uuid;

pub async fn create_post(pool: &PgPool, new_post: &NewPost) -> Result<Post, sqlx::Error> {
    let id = Uuid::new_v4();

    let post = sqlx::query_as!(
        Post,
        r#"
        INSERT INTO posts (id, title, content)
        VALUES ($1, $2, $3)
        RETURNING id, title, content, created_at
        "#,
        id,
        new_post.title,
        new_post.content
    )
    .fetch_one(pool)
    .await?;

    Ok(post)
}

pub async fn get_all_posts(pool: &PgPool) -> Result<Vec<Post>, sqlx::Error> {
    sqlx::query_as!(
        Post,
        r#"
        SELECT id, title, content, created_at
        FROM posts
        ORDER BY created_at DESC
        "#,
    )
    .fetch_all(pool)
    .await
}

pub async fn get_post_by_id(pool: &PgPool, post_id: Uuid) -> Result<Option<Post>, sqlx::Error> {
    let post = sqlx::query_as!(
        Post,
        r#"
        SELECT id, title, content, created_at
        FROM posts
        WHERE id = $1
        "#,
        post_id
    )
    .fetch_optional(pool)
    .await?;

    Ok(post)
}

pub async fn delete_post(pool: &PgPool, post_id: Uuid) -> Result<bool, sqlx::Error> {
    let result = sqlx::query!(
        r#"
        DELETE FROM posts
        WHERE id = $1
        "#,
        post_id
    )
    .execute(pool)
    .await?;

    Ok(result.rows_affected() > 0)
}

#[allow(clippy::too_many_lines)]
#[must_use]
pub fn mock_food_data() -> Vec<Food> {
    vec![
        Food {
            title: "Soft-boiled Eggs",
            slug: "soft-boiled-eggs",
            image_gallery: vec!["perfectly-runny-soft-boiled-eggs.jpg"],
            main_ingredients: vec!["Eggs", "Water", "Salt"],
            cooking_method: "boiling",
            equipment: vec!["pot", "tongs", "bowl"],
            estimated_time_required_in_minutes: 10,
            one_serving_visual_reference: "1 egg",
            one_serving_weight_reference_in_grams: 50,
            health_profile: "healthy",
            preparation_difficulty: "simple",
            cooking_instructions: vec![
                "Bring water to a boil in a pot.",
                "Gently lower the eggs into the boiling water using tongs.",
                "Boil for 6 minutes for runny yolks or 8 minutes for slightly firmer yolks.",
                "Remove the eggs with tongs and place them in a bowl of cold water to stop the cooking process.",
                "Peel and enjoy!",
            ],
            diet_friendly_to: vec![
                "low-carb",
                "keto",
                "keto-friendly",
                "carnivore",
                "paleo",
                "vegetarian",
                "gluten-free",
                "dairy-free",
                "nut-free",
            ],
        },
        Food {
            title: "Scrambled Eggs",
            slug: "scrambled-eggs",
            image_gallery: vec!["scrambled-eggs.jpg"],
            main_ingredients: vec!["Eggs", "Animal fat (the more the merrier)", "Salt"],
            cooking_method: "pan-frying",
            equipment: vec!["skillet", "spatula", "bowl"],
            estimated_time_required_in_minutes: 15,
            one_serving_visual_reference: "2 eggs",
            one_serving_weight_reference_in_grams: 100,
            health_profile: "healthy",
            preparation_difficulty: "simple",
            cooking_instructions: vec![
                "Crack the eggs into a bowl and whisk until well combined.",
                "Heat a skillet over medium heat and melt the butter.",
                "Pour the eggs into the skillet and let them cook undisturbed for a few seconds.",
                "Gently stir the eggs with a spatula, pushing them from the edges to the center.",
                "Continue to cook and stir until the eggs are softly set and slightly runny in places.",
                "Season with salt and pepper to taste, then remove from heat and serve immediately.",
            ],
            diet_friendly_to: vec![
                "low-carb",
                "keto",
                "keto-friendly",
                "carnivore",
                "paleo",
                "vegetarian",
                "gluten-free",
                "dairy-free",
                "nut-free",
            ],
        },
        Food {
            title: "Pan-fried burger patties",
            slug: "pan-fried-burger-patties",
            image_gallery: vec!["pan-fried-burgers.jpg"],
            main_ingredients: vec![
                "Ground beef",
                "Salt",
                "Animal fat (for greasing the skillet)",
            ],
            cooking_method: "pan-frying",
            equipment: vec!["skillet", "spatula"],
            estimated_time_required_in_minutes: 30,
            one_serving_visual_reference: "1 patty",
            one_serving_weight_reference_in_grams: 200,
            health_profile: "healthy",
            preparation_difficulty: "simple",
            cooking_instructions: vec![
                "Preheat a skillet over medium-high heat and lightly grease it with animal fat.",
                "Divide the ground beef into equal portions and shape them into patties.",
                "Season both sides of the patties with salt.",
                "Place the patties in the hot skillet and cook for about 4-5 minutes on each side for medium doneness, or adjust the time to your preferred level of doneness.",
                "Remove the patties from the skillet and let them rest for a few minutes before serving.",
            ],
            diet_friendly_to: vec![
                "low-carb",
                "keto",
                "keto-friendly",
                "carnivore",
                "paleo",
                "gluten-free",
                "dairy-free",
                "nut-free",
            ],
        },
        Food {
            title: "Italian pork sausage",
            slug: "italian-pork-sausage",
            image_gallery: vec!["italian-pork-sausage.jpg"],
            main_ingredients: vec!["Ground pork", "Salt", "Italian seasoning"],
            cooking_method: "pan-frying",
            equipment: vec!["skillet", "spatula"],
            estimated_time_required_in_minutes: 30,
            one_serving_visual_reference: "1 sausage link",
            one_serving_weight_reference_in_grams: 150,
            health_profile: "healthy",
            preparation_difficulty: "simple",
            cooking_instructions: vec![
                "Preheat a skillet over medium-high heat and lightly grease it with animal fat.",
                "Shape the ground pork into sausage links or patties.",
                "Season the pork with salt and Italian seasoning.",
                "Place the sausage links or patties in the hot skillet and cook for about 5-7 minutes on each side, or until they are cooked through and have a nice browned exterior.",
            ],
            diet_friendly_to: vec![
                "low-carb",
                "keto",
                "keto-friendly",
                "carnivore",
                "paleo",
                "gluten-free",
                "dairy-free",
                "nut-free",
            ],
        },
        Food {
            title: "Medovik",
            slug: "medovik",
            image_gallery: vec!["medovik.jpg"],
            main_ingredients: vec!["Honey", "Flour", "Eggs", "Butter", "Sour cream"],
            cooking_method: "baking",
            equipment: vec!["oven", "mixing bowls", "whisk", "baking pans"],
            estimated_time_required_in_minutes: 120,
            one_serving_visual_reference: "1 slice",
            one_serving_weight_reference_in_grams: 150,
            health_profile: "indulgent",
            preparation_difficulty: "intermediate",
            cooking_instructions: vec![
                "Preheat the oven to 350°F (175°C).",
                "In a mixing bowl, combine honey, flour, eggs, and butter to form a dough.",
                "Divide the dough into equal portions and roll each portion into a thin circle.",
                "Place the rolled dough circles on baking pans and bake for about 5-7 minutes until they are golden brown.",
                "Allow the baked layers to cool completely.",
                "In a separate bowl, whisk together sour cream and a bit of honey to make the filling.",
                "Assemble the cake by layering the baked dough circles with the sour cream filling in between each layer.",
                "Refrigerate the assembled cake for several hours or overnight to allow the flavors to meld and the cake to set before slicing and serving.",
            ],
            diet_friendly_to: vec!["Vegetarian"],
        },
        Food {
            title: "Cheesecake",
            slug: "cheesecake",
            image_gallery: vec!["cheesecake.jpg"],
            main_ingredients: vec!["Cream cheese", "Sugar", "Eggs", "Graham cracker crust"],
            cooking_method: "baking",
            equipment: vec!["oven", "mixing bowls", "whisk", "springform pan"],
            estimated_time_required_in_minutes: 90,
            one_serving_visual_reference: "1 slice",
            one_serving_weight_reference_in_grams: 200,
            health_profile: "indulgent",
            preparation_difficulty: "intermediate",
            cooking_instructions: vec![
                "Preheat the oven to 325°F (160°C).",
                "In a mixing bowl, beat the cream cheese and sugar until smooth and creamy.",
                "Add the eggs one at a time, beating well after each addition.",
                "Pour the cream cheese mixture over the graham cracker crust in a springform pan.",
                "Bake for about 60-70 minutes, or until the center is set and the edges are lightly browned.",
                "Turn off the oven and let the cheesecake cool in the oven with the door slightly open for about an hour.",
                "Refrigerate the cheesecake for at least 4 hours or overnight before slicing and serving.",
            ],
            diet_friendly_to: vec!["Vegetarian"],
        },
    ]
}
