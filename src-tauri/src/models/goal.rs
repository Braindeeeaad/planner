

use sqlx::sqlite::SqlitePool;
use sqlx::{FromRow};
use uuid::Uuid;
use serde_json::json;

use crate::core::graph_components::{Action,NodeType};
#[derive(FromRow)]
pub struct Goal {
    id: String,
    node_id:String,
    title: String,
    target_date: String,
    status: String,
}


impl Action for Goal{
    fn get_uuid(&self)->&str {
        return &self.id
    }
    fn get_json_str(&self)->String {
        self.get_json_str()
    }
    fn get_node_type(&self)->NodeType {
        NodeType::GOAL
    }
}

impl Goal {
    pub fn new(id:&str, title: String, target_date: String, status: String) -> Self {
        Self {
            id: String::from(id),
            node_id: String::from(id),
            title,
            target_date,
            status,
        }
    }
    pub fn get_id(&self)->&str{
        &self.id
    }
    pub fn get_json_str(&self)->String{
        let self_json = json!({
            "type":"GOAL",
            "id":self.id, 
            "node_id":self.node_id,
            "title":self.title, 
            "target_date":self.target_date, 
            "status":self.status
        }); 
        self_json.to_string() 
    }
}

pub async fn upload_goal(pool: &SqlitePool, goal: Goal) -> anyhow::Result<Goal> {
    let query = "INSERT INTO goals (id,node_id,title,target_date,status) VALUES ($1,$2,$3,$4,&5)";
    sqlx::query(query)
        .bind(&goal.id)
        .bind(&goal.node_id)
        .bind(&goal.title)
        .bind(&goal.target_date)
        .bind(&goal.status)
        .execute(pool)
        .await?;

    Ok(Goal {
        id: goal.id,
        node_id:goal.node_id,
        title: goal.title,
        target_date: goal.target_date,
        status: goal.status,
    })
}

pub async fn delete_goal(pool:&SqlitePool, goal:Goal)->anyhow::Result<()>{
    let query = "DELETE FROM goals WHERE id=$1";
    sqlx::query(query)
        .bind(goal.id)
        .execute(pool)
        .await?;
    Ok(())
}


pub async fn get_goals(pool: &SqlitePool) -> anyhow::Result<Vec<Goal>> {
    let goals =
        sqlx::query_as::<_, Goal>(r#"SELECT id,title,target_date,status FROM goals"#)
            .fetch_all(pool)
            .await?;
    Ok(goals)
}
