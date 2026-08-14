

use sqlx::sqlite::SqlitePool;
use sqlx::{FromRow};
use uuid::Uuid;

use crate::core::dag::Action;
#[derive(FromRow)]
pub struct Goal {
    id: String,
    title: String,
    target_date: String,
    status: String,
}


impl Action for Goal{
    fn get_uuid(&self)->&str {
        return &self.id
    }
}

impl Goal {
    pub fn new(title: String, target_date: String, status: String) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            title,
            target_date,
            status,
        }
    }
    pub fn get_id(&self)->&str{
        &self.id
    }
}

pub async fn upload_goal(pool: &SqlitePool, goal: Goal) -> anyhow::Result<Goal> {
    let query = "INSERT INTO goals (id,title,target_date,status) VALUES ($1,$2,$3,$4)";
    sqlx::query(query)
        .bind(&goal.id)
        .bind(&goal.title)
        .bind(&goal.target_date)
        .bind(&goal.status)
        .execute(pool)
        .await?;

    Ok(Goal {
        id: goal.id,
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
