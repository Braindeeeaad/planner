use sqlx::FromRow; 

#[derive(Debug,FromRow)] 
pub struct habits{
    id: String, 
    goal_id: Option<String>, 
    title: String,
    frequency: String,
    target_time: u32, 
    streak_count: u32,
}

