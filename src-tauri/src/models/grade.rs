use sqlx::FromRow; 

#[derive(Debug,FromRow)] 
pub struct grade_item{
    id: String, 
    event_context_id: String, 
    title: String,
    weight: f32,
    earned_score: f32,
    max_score: f32, 
    target_grade: f32
}

#[derive(Debug,FromRow)] 
pub struct user_ability_profiles{
    category: String, 
    velocity_multiplier: f32, 
    notes: String,
}



