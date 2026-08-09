use sqlx::FromRow; 

#[derive(Debug,FromRow)] 
pub struct goal{
    id: String, 
    title: String, 
    target_date: String,
    status: String
}


pub fn create_goal(){}

pub fn get_goals(){}
