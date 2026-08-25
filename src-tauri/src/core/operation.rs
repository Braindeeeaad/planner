use serde::{Deserialize, Serialize};
use sqlx::{Sqlite, Transaction};
use std::collections::{HashMap};


use crate::models::habit::{Habit}; 
use crate::models::task::{Task}; 
use crate::models::goal::{Goal};

use crate::core::dag::{Dag};
use crate::core::graph_components::{DagError, Node, delete_node, get_node, upload_node};
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Op {
    AddTask {
        task:Task,
        x: Option<f32>,
        y: Option<f32>,
    },
    AddHabit {
        habit:Habit,
        x: Option<f32>,
        y: Option<f32>,
    },
    AddGoal {
        goal:Goal,
        x: Option<f32>,
        y: Option<f32>,
    },
    RemoveNode {
        id: String,
    },
    AddEdge {
        predecessor_id: String,
        successor_id: String,
    },
    RemoveEdge {
        predecessor_id: String,
        successor_id: String,
    },
    MoveNode {
        id: String,
        x: f32,
        y: f32,
    },
    ModifyNode {
        id: String,
        json_str: String,
    },
    Batch {
        ops: Vec<Op>,
    },
}


async fn apply_op(dag_map: &mut HashMap<String, Dag>,pool:&sqlx::SqlitePool, op: &Op) -> anyhow::Result<()> {
    match op {
        Op::AddTask {task, x, y } => {
            upload_node(pool, task.clone(), x.clone(), y.clone());
        }
        Op::AddHabit {habit, x, y } => {
            upload_node(pool, habit.clone(), x.clone(), y.clone());
        }
        Op::AddGoal {goal, x, y } => {
            upload_node(pool, goal.clone(), x.clone(), y.clone());
        }
        Op::AddEdge { predecessor_id, successor_id } => {
            let succ_node = get_node(pool, successor_id).await?;
            let pred_node = get_node(pool, predecessor_id).await?;
            
            let goal_id = match pred_node.item.get_goal_id(){
                Some(goal_id)=>goal_id, 
                None => return Err(DagError::EdgeError { message: ("Predecessor node needs to be connected to graph".to_string()) })?
            };

            let dagr = dag_map.get(&goal_id).unwrap();
            dagr.add_edge(successor_id.clone(), predecessor_id.clone());
            dagr.add_node_if_not_present(succ_node);
            
            
        }
        Op::RemoveNode { id } =>{
            let node = get_node(pool,id).await?;
            let goal_id = match node.item.get_goal_id() {
                Some(goal_id) => goal_id,          // now goal_id: String
                None => {
                    return Err(DagError::EdgeError {
                        message: "Predecessor node needs to be connected to graph".to_string(),
                    })?
                }
            };

            
            if goal_id{
                //TODO: add error handling here

                let dagr = *dag_map.get_mut(&goal_id.unwrap()).unwrap();
                dagr.delete_node(id);
            }
            else{
                delete_node(pool, node);
            }
        }
        Op::MoveNode { id, x, y } => {
            
        }
        Op::Batch { ops } => {
            for sub_op in ops {
                Box::pin(apply_op(dag_map, pool,op)).await?; // recursion needs boxing (async fn)
            }
        }
        // RemoveNode, RemoveEdge, RenameNode similarly...
        _ => todo!(),
    }
    Ok(())
}


