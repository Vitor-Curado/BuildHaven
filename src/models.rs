use chrono::{DateTime, Utc};
use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Clone)]
pub struct NewUser {
    pub username: String,
    pub email: String,
    pub password_hash: String,
}

#[derive(Debug, Clone)]
pub struct PublicUser {
    pub id: Uuid,
    pub username: String,
    pub email: String,
}

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct Post {
    pub id: Uuid,
    pub title: String,
    pub content: String,
    //pub excerpt: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Clone)]
pub struct NewPost {
    pub title: String,
    pub content: String,
    //pub excerpt: String
}
#[derive(Serialize, Clone)]
pub struct Food {
    pub title: &'static str,
    pub slug: &'static str,
    pub image_gallery: Vec<&'static str>,
    pub main_ingredients: Vec<&'static str>,
    pub cooking_method: &'static str,
    pub equipment: Vec<&'static str>,
    pub estimated_time_required_in_minutes: u32,
    pub one_serving_visual_reference: &'static str,
    pub one_serving_weight_reference_in_grams: u32,

    pub health_profile: &'static str,
    pub preparation_difficulty: &'static str,
    pub cooking_instructions: Vec<&'static str>,
    pub diet_friendly_to: Vec<&'static str>,
}

/*
#[derive(Serialize)]
pub struct BlogPost {
    pub title: &'static str,
    pub slug: &'static str,
    pub published_date: &'static str,
    pub last_updated_date: &'static str,
    pub content: &'static str
}

#[derive(Serialize)]
pub struct Project {
    pub title: &'static str,
    pub slug: &'static str,
    pub technology_stack: Vec<&'static str>,
    pub status: &'static str,
    pub features: Vec<&'static str>
}

#[derive(Serialize)]
pub struct Job {
    pub title: &'static str,
    pub company: &'static str,
    pub location: &'static str,
    pub start_date: &'static str,
    pub end_date: Option<&'static str>,
    pub job_duties: Vec<&'static str>
}
*/
