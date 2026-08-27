use serde::{Deserialize, Serialize};
use sqlx::{Sqlite, Transaction};
use std::collections::{HashMap};


use crate::models::habit::{Habit}; 
use crate::models::task::{Task}; 
use crate::models::goal::{Goal};

use crate::core::dag::{Dag};
use crate::core::graph_components::{Action, DagError, Node, NodeType, delete_node, get_node, upload_node};
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


pub async fn apply_op(dag_map: &mut HashMap<String, Dag>,pool:&sqlx::SqlitePool, op: &Op) -> anyhow::Result<Vec<Op>> {
    let mut inverse_ops:Vec<Op> = Vec::new();
    match op {
        Op::AddTask {task, x, y } => {
            upload_node(pool, task.clone(), x.clone(), y.clone());
            //calculation inverse operation
            inverse_ops.push(Op::RemoveNode { id: (String::from(task.get_uuid()))});
        }
        Op::AddHabit {habit, x, y } => {
            upload_node(pool, habit.clone(), x.clone(), y.clone());
            //calculation inverse operation
            inverse_ops.push(Op::RemoveNode { id: (String::from(habit.get_uuid()))});
        }
        Op::AddGoal {goal, x, y } => {
            upload_node(pool, goal.clone(), x.clone(), y.clone());
            //calculation inverse operation
            inverse_ops.push(Op::RemoveNode { id: (String::from(goal.get_uuid()))});
        }
        Op::AddEdge { predecessor_id, successor_id } => {
            let succ_node = get_node(pool, successor_id).await?;
            let pred_node = get_node(pool, predecessor_id).await?;
            
            let goal_id = match pred_node.item.get_goal_id(){
                Some(goal_id)=>goal_id, 
                None => return Err(DagError::EdgeError { message: ("Predecessor node needs to be connected to graph".to_string()) })?
            };


            if dag_map.get(&goal_id).is_some(){
                //TODO: add error handling here
                let dagr = dag_map.get_mut(&goal_id).unwrap();
                dagr.add_edge(successor_id.clone(), predecessor_id.clone());
                dagr.add_node_if_not_present(succ_node);
            }

            //calculating inverse operation

            inverse_ops.push(Op::RemoveEdge { predecessor_id: (predecessor_id.to_string()), successor_id: (successor_id.to_string()) });

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

            
            //calculating inverse operation
            match node.node_type{
                NodeType::GOAL => {
                    let goal:Goal = serde_json::from_value(node.item.get_json_fields().unwrap()).unwrap();
                    inverse_ops.push(Op::AddGoal { goal, x: (node.x), y: (node.y)})
                },
                NodeType::TASK => {
                    let task:Task = serde_json::from_value(node.item.get_json_fields().unwrap()).unwrap();
                    inverse_ops.push(Op::AddTask { task, x: (node.x), y: (node.y)})
                },
                NodeType::HABIT => {
                    let habit:Habit = serde_json::from_value(node.item.get_json_fields().unwrap()).unwrap();
                    inverse_ops.push(Op::AddHabit { habit, x: (node.x), y: (node.y)})
                },
            }




            //deleting node from dag or from db
            if dag_map.get(&goal_id).is_some(){
                //TODO: add error handling here
                let dagr = dag_map.get_mut(&goal_id).unwrap();
                dagr.delete_node(id);
            }
            else{
                delete_node(pool, node);
            }

            
            
        }
        Op::MoveNode { id, x, y } => {
            let mut node = get_node(pool, id).await?; 
            node.set_coords(Some(x.clone()), Some(y.clone()));
            node.save_coords(pool);
            inverse_ops.push(Op::MoveNode { id: (id.to_string()), x: (-x), y: (-y) });
        }
        Op::Batch { ops } => {
            for sub_op in ops {
                let op = Box::pin(apply_op(dag_map, pool,op)).await?; // recursion needs boxing (async fn)
                inverse_ops.extend(op);
            }
        }
        // RemoveNode, RemoveEdge, RenameNode similarly...
        _ => todo!(),
    }
    Ok(inverse_ops)
}


