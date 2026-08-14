use std::vec;
use std::collections::HashMap;
use sqlx::sqlite::SqlitePool;

//Would help if I maintained my own dag datastructure in mem to manipulate then save periodically to the db 
//Would need a get_task_dependencies, and get_tasks by goal_id, as well as event_contexts
//Given tasks,habits,event_contexts, and task_dependencies Make apporpriate nodes and edges
//Make sure there's logic that prevents cross-edges that break dag and back-edges 
//Add Topological sort
use crate::models::habit::{Habit,upload_habit,delete_habit,get_habits}; 
use crate::models::task::{Task, delete_task, get_task_dependencies, get_tasks, upload_task}; 
use crate::models::goal::{Goal,upload_goal,delete_goal,get_goals};
use crate::db::connection::{establish_connection};

pub trait Action{
    fn get_uuid(&self)->&str;
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
struct Dag{
    

    pool:SqlitePool,
    goal:Goal,

    nodes:HashMap<String,Node>, 
    //Screw loop cycle checks, instead write user changes to db first
    //Then copy it down into in mem-dag(write-through persistence)
    //Can remove the db::connection::{States} usage throughout and just have 
    //Normal structs 
    

    //Can map task_dependencies to here
    //If user deletes task_dependecy, first delete db task_dependency 
    //Then query predecessor into successor Hashmap and delete respective succesor 
    //Vice versa for predocessors, query in sucessor from task_dependency into predecssor, and delete respective predecessor 
    
    successors: HashMap<String, Vec<String>>, 

    predecessors: HashMap<String, Vec<String>>
}


impl Dag{
    pub async fn new(goal:Goal)->anyhow::Result<Self>{
        let pool =  establish_connection().await?;
        Ok(
            Self{   
            pool,
            goal,
            nodes:HashMap::new(), 
            successors:HashMap::new(), 
            predecessors:HashMap::new() 
            
        })

    }
    async fn download_dag(&mut self)->anyhow::Result<()>{ 
        let tasks = get_tasks(&self.pool, Some(self.goal.get_id())).await?;
        let habits = get_habits(&self.pool, Some(self.goal.get_id())).await?;
        let task_dependencies = get_task_dependencies(&self.pool, self.goal.get_id()).await?;
        
        for task in tasks{
            self.nodes.insert(String::from(task.get_uuid()),Node::new(task));
        }
        for habit in habits{
            self.nodes.insert(String::from(habit.get_uuid()),Node::new(habit));
        }
        for task_dep in task_dependencies{
            let succ_id = String::from(task_dep.successor_id); 
            let pred_id = String::from(task_dep.predecessor_id); 
            let sucessor_vec = self.successors.entry(pred_id.clone()).or_insert(Vec::new());
            let predecessor_vec = self.predecessors.entry(succ_id.clone()).or_insert(Vec::new());
            sucessor_vec.push(succ_id);
            predecessor_vec.push(pred_id);

        }
        Ok(())
    }

    pub async fn make_edge(){

    }
    pub async fn make_task(){

    }
    pub async fn make_habit(){

    }

}