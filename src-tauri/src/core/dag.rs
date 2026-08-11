use std::vec;
use std::collections::HashMap;

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
    item: Box<dyn Action>
} 

impl Node{
    pub fn new(action: impl Action + 'static) -> Self{
        Self{
            item: Box::new(action),
        }
    }

}


struct Edge{
    edge:(String,String)
}

impl Edge{
    pub fn new(node_hash1:String,node_hash2:String)->Self{
        Self{
            edge:(node_hash1,node_hash2)
        }
    }
}



//We can make a hash-map relating uuid:Node
//1.Need function that downloads Nodes(Tasks/Habits) from sql and configures hashmap 
//2.Need function that downloads Edges(task_dependencies) and configures list of Edges
//3.Need to implement appropriate upload and delete functions for task dependencies 
//4.We could make another hashmap using the uuids of the first element of the edges, and the respective 
//  vector of edges that exist there
//5.Need a loop cycle that syncs the node and edge states with the sql database.  
struct Dag{
    
    pub nodes:HashMap<String,Node>, 

    //Screw loop cycle checks, instead write user changes to db first
    //Then copy it down into in mem-dag(write-through persistence)
    //Can remove the db::connection::{States} usage throughout and just have 
    //Normal structs 
    

    //Can map task_dependencies to here
    //If user deletes task_dependecy, first delete db task_dependency 
    //Then query predecessor into successor Hashmap and delete respective succesor 
    //Vice versa for predocessors, query in sucessor from task_dependency into predecssor, and delete respective predecessor 
    
    pub successors: HashMap<String, Vec<String>>, 

    pub predecessors: HashMap<String, Vec<String>>
}