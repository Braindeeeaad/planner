use sqlx::FromRow; 
use uuid::Uuid;
use sqlx::{SqlitePool};
use serde_json::json;

use crate::core::graph_components::{Action,NodeType};



#[derive(Debug,FromRow,Clone)] 
pub struct Habit{
    id: String, 
    node_id: String, 
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
    fn get_json_str(&self)->String {
        self.get_json_str()
    }
    fn get_node_type(&self)->NodeType {
        NodeType::HABIT
    }
}

impl Habit{
    pub fn new(id:&str, title:String, frequency: String, target_time:u32 ,goal_id:Option<String>,)->Self{
        Self{
            id:String::from(id),
            node_id:String::from(id), 
            goal_id,
            title, 
            frequency, 
            target_time, 
            streak_count:0
        }
    }
    pub fn get_json_str(&self)->String{
        let self_json = json!({
            "type":"HABIT",
            "id":self.id, 
            "node_id":self.node_id,
            "goal_id":self.goal_id, 
            "title":self.title, 
            "target_time":self.target_time,
            "streak_count":self.streak_count
        }); 
        self_json.to_string() 
    }
}


pub async fn upload_habit(pool: &SqlitePool, habit: &Habit ) -> anyhow::Result<()> {
    let query = "INSERT INTO habits (id,node_id,title,frequency,target_time,streak_count,goal_id) VALUES ($1,$2,$3,$4,&5,&6,&7)";
    sqlx::query(query)
        .bind(&habit.id)
        .bind(&habit.node_id)
        .bind(&habit.title)
        .bind(&habit.frequency)
        .bind(&habit.target_time)
        .bind(&habit.streak_count)
        .bind(&habit.goal_id)
        .execute(pool)
        .await?;
    Ok(())
}



pub async fn delete_habit(pool:&SqlitePool, habit:Habit)->anyhow::Result<()>{
    let query = "DELETE FROM habits WHERE id=$1";
    sqlx::query(query)
        .bind(habit.id)
        .execute(pool)
        .await?;
    Ok(())
}


pub async fn get_habits(pool: &SqlitePool, group_id:Option<&str>) -> anyhow::Result<Vec<Habit>> {
    let habits =
        sqlx::query_as::<_, Habit>(r#"SELECT id,node_id,goal_id,title,frequency,target_time,streak_count FROM habits
                                                   WHERE ($1 IS NULL or group_id = $1)"#)
            .bind(group_id)
            .fetch_all(pool)
            .await?;
    Ok(habits)
}
