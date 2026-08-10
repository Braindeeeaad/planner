//Would help if I maintained my own dag datastructure in mem to manipulate then save periodically to the db 
//Would need a get_task_dependencies, and get_tasks by goal_id, as well as event_contexts
//Given tasks,habits,event_contexts, and task_dependencies Make apporpriate nodes and edges
//Make sure there's logic that prevents cross-edges that break dag and back-edges 
//Add Topological sort
use crate::models::habit::Habit; 
use crate::models::task::Task; 
use crate::models::goal::Goal;

pub trait Action{
    fn get_uuid(&self)->&str;
    fn upload(&self)->Result<(),sqlx::Error>;
}
struct Node{
    
} 

struct Dag{
    
}