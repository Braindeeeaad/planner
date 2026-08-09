use sqlx::sqlite::SqlitePool;
use sqlx::FromRow;

#[derive(Debug, FromRow)]
pub struct GradeItem {
    id: String,
    event_context_id: String,
    title: String,
    weight: f32,
    earned_score: f32,
    max_score: f32,
    target_grade: f32,
}

#[derive(Debug, FromRow)]
pub struct UserAbilityProfile {
    category: String,
    velocity_multiplier: f32,
    notes: String,
}

pub async fn get_user_ability_profile(
    category: &str,
    pool: &SqlitePool,
) -> anyhow::Result<Option<UserAbilityProfile>> {
    let profile = sqlx::query_as::<_, UserAbilityProfile>(
        r#"SELECT category,velocity_multiplier,notes FROM user_ability_profiles WHERE category = $1"#,
    )
    .bind(category)
    .fetch_optional(pool)
    .await?;
    Ok(profile)
}

pub async fn update_user_velocity(
    category: &str,
    velocity_multiplier: f32,
    pool: &SqlitePool,
) -> anyhow::Result<()> {
    let query = "UPDATE user_ability_profiles SET velocity_multiplier = $1 WHERE category = $2";
    sqlx::query(query)
        .bind(velocity_multiplier)
        .bind(category)
        .execute(pool)
        .await?;
    Ok(())
}