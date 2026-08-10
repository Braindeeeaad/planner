use sqlx::FromRow; 
use crate::core::dag::Action;
#[derive(Debug,FromRow)] 
pub struct Habit{
    id: String, 
    goal_id: Option<String>, 
    title: String,
    frequency: String,
    target_time: u32, 
    streak_count: u32,
}

impl Action for Habit{
    fn get_uuid(&self)->&str {
        &self.id
    }
    fn upload(&self)->Result<(),sqlx::Error> {
        Ok(())
    }
}