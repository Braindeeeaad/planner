use sqlx::FromRow; 
use uuid::Uuid;
use sqlx::{SqlitePool};



use crate::core::dag::Action;



#[derive(Debug,FromRow,Clone)] 
pub struct Habit{
    id: String, 
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
}

impl Habit{
    pub fn new(title:String, frequency: String, target_time:u32 ,goal_id:Option<String>,)->Self{
        Self{
            id:Uuid::new_v4().to_string(), 
            goal_id,
            title, 
            frequency, 
            target_time, 
            streak_count:0
        }
    }
}


pub async fn upload_habit(habit: Habit, pool: &SqlitePool) -> anyhow::Result<()> {
    let query = "INSERT INTO habits (id,title,frequency,target_time,streak_count,goal_id) VALUES ($1,$2,$3,$4,&5,&6)";
    sqlx::query(query)
        .bind(&habit.id)
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
        sqlx::query_as::<_, Habit>(r#"SELECT id,goal_id,title,frequency,target_time,streak_count FROM habits
                                                   WHERE ($1 IS NULL or group_id = $1)"#)
            .bind(group_id)
            .fetch_all(pool)
            .await?;
    Ok(habits)
}
