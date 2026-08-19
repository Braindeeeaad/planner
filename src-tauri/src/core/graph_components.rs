use std::vec;
use std::collections::{HashMap,VecDeque};
use serde::Serialize;
use sqlx::Sqlite;
use sqlx::Row;
use sqlx::sqlite::SqlitePool;
use uuid::Uuid;

use thiserror::Error;
//Would help if I maintained my own dag datastructure in mem to manipulate then save periodically to the db 
//Would need a get_task_dependencies, and get_tasks by goal_id, as well as event_contexts
//Given tasks,habits,event_contexts, and task_dependencies Make apporpriate nodes and edges
//Make sure there's logic that prevents cross-edges that break dag and back-edges 
//Add Topological sort
use crate::models::habit::{Habit,upload_habit,delete_habit,get_habits}; 
use crate::models::task::{Task, TaskDependency, delete_task, get_task_dependencies, get_tasks, upload_task, upload_task_dependency}; 
use crate::models::goal::{Goal,upload_goal,delete_goal,get_goals};
use crate::db::connection::{establish_connection};

#[derive(Error,Debug)]
pub enum DagError{
    #[error("Edge error: {message}")]
    EdgeError{message:String},

    #[error("Node error: {message}")]
    NodeError{message:String},
}

#[derive(Serialize)]
pub enum NodeType{
    HABIT, 
    GOAL, 
    TASK
}

#[typetag::serde(tag="type")]
pub trait Action{
    fn get_uuid(&self)->&str;
    fn get_json_str(&self)->String;
    fn get_node_type(&self)->NodeType; 
}

#[derive(Serialize)]
pub struct Node{
    id:String,
    node_type: NodeType, 
    x:Option<f32>, 
    y:Option<f32>,
    pub item: Box<dyn Action>
} 

impl Node{

    pub fn new(action: impl Action + 'static,x:Option<f32>,y:Option<f32>) -> Self{
        let action_pntr = Box::new(action);
        let node_type = action_pntr.get_node_type();
        let id = String::from(action_pntr.get_uuid()); 
        
        Self{
            id,
            item:action_pntr, 
            node_type,
            x,
            y
        }
    }

    pub async fn fetch_coords(&mut self,pool:&SqlitePool)->anyhow::Result<()>{
        let query = "SELECT (x,y) FROM nodes WHERE id=$1";
        let row = sqlx::query(query)
            .bind(&self.id)
            .fetch_one(pool)
            .await?;
        self.x = row.get("x"); 
        self.y = row.get("y");
        Ok(())
    }

    
}


pub async fn upload_node(pool:&SqlitePool,action: impl Action + 'static,x:Option<f32>,y:Option<f32>)->anyhow::Result<Node>{
    let query = "INSERT INTO nodes (id,x,y) VALUES ($1,$2,$3)";
    let node = Node::new(action, x, y);
    sqlx::query(query)
        .bind(&node.id)
        .bind(&node.x)
        .bind(&node.y)
        .execute(pool)
        .await?;
    Ok(node)
}

pub async fn delete_node(pool:&SqlitePool, node:Node)->anyhow::Result<()>{
    let query = "DELETE FROM nodes WHERE (id = $1)"; 
    sqlx::query(query)
        .bind(node.id)
        .execute(pool)
        .await?;
    Ok(())
}


