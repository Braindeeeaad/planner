use sqlx::FromRow; 

#[derive(Debug,FromRow)]
pub struct task{
    id: String, 
    
    goal_id: Optional<String>, 
    event_context_d: Optional<String>, 
    
    title: String, 
    task_type: String, 
    
    base_duration: u32, 
    schedule_start: String,
    schedule_end: String,
    
    urgency_score: f32, 
    importance_score: f32, 
    priority_weight: f32,
    
    status: String
}

#[derive(Debug,FromRow)]
pub struct task_dependancies{
    predecessor_id: String, 
    successor_id: String,
}



#[derive(Debug,FromRow)]
pub struct task_feedback{
    id: String, 
    task_id: String,
    estimated_duration: u32, 
    actual_duration: u32, 
    user_sentiment: String, 
    completion_quality: u8,
    created_at: String,
}




pub fn create_task(){}

pub fn get_task_by_goal(){}