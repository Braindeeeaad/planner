use sqlx::sqlite::SqlitePool;
use sqlx::{FromRow, Row, sqlite::SqliteRow};
use std::ffi::NulError;
use std::marker::PhantomData;
use uuid::Uuid;
use crate::core::dag::Action;
use std::any::TypeId;


#[derive(Clone,FromRow)]
pub struct Task {
    id: String,
    goal_id: Option<String>,
    event_context_id: Option<String>,
    title: String,
    task_type: String,
    base_duration: u32,
    schedule_start: String,
    schedule_end: String,
    urgency_score: f64,
    importance_score: f64,
    priority_weight: f64,
    status: String,
}


/*

    Task trait implementation


*/


impl Action for Task{
    fn get_uuid(&self)->&str {
        &self.id
    }
}


impl Task{
    pub fn new(
        goal_id: Option<String>,
        event_context_id: Option<String>,
        title: String,
        task_type: String,
        base_duration: u32,
        schedule_start: String,
        schedule_end: String,
        urgency_score: f64,
        importance_score: f64,
        priority_weight: f64,
        status: String,
    ) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            goal_id,
            event_context_id,
            title,
            task_type,
            base_duration,
            schedule_start,
            schedule_end,
            urgency_score,
            importance_score,
            priority_weight,
            status,
        }
    }
}

pub async fn upload_task(task: Task, pool: &SqlitePool) -> anyhow::Result<()> {
    let query = "INSERT INTO tasks
                       (id,goal_id,event_context_id,title,task_type,base_duration,schedule_start,schedule_end,urgency_score,importance_score,priority_weight,status)
                       VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,$12)";
    sqlx::query(query)
        .bind(&task.id)
        .bind(&task.goal_id)
        .bind(&task.event_context_id)
        .bind(&task.title)
        .bind(&task.task_type)
        .bind(&task.base_duration)
        .bind(&task.schedule_start)
        .bind(&task.schedule_end)
        .bind(task.urgency_score)
        .bind(task.importance_score)
        .bind(task.priority_weight)
        .bind(&task.status)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn delete_task(pool:&SqlitePool, task:Task)->anyhow::Result<()>{
    let query = "DELETE FROM tasks WHERE id=$1";
    sqlx::query(query)
        .bind(task.id)
        .execute(pool)
        .await?;
    Ok(())
}


pub async fn get_tasks(
    pool: &SqlitePool,
    goal_id: Option<&str>,
) -> anyhow::Result<Vec<Task>> {
    let tasks = sqlx::query_as::<_, Task>(
        r#"
        SELECT id,goal_id,event_context_id,
               title,task_type,base_duration,
               schedule_start,schedule_end,
               urgency_score,importance_score,
               priority_weight,status 
        FROM tasks WHERE ($1 IS NULL or group_id = $1)"#,
    )
    .bind(goal_id)
    .fetch_all(pool)
    .await?;
    Ok(tasks)
}



#[derive(Debug, FromRow)]
pub struct TaskDependency {
    predecessor_id: String,
    successor_id: String,
    goal_id: String
}

pub async fn upload_task_dependency(
    pool: &SqlitePool,
    predecessor_id: &str,
    successor_id: &str,
    goal_id: &str,
) -> anyhow::Result<()> {
    let query = "INSERT INTO task_dependencies (predecessor_id,successor_id,goal_id) VALUES ($1,$2,$3)";
    sqlx::query(query)
        .bind(predecessor_id)
        .bind(successor_id)
        .bind(goal_id)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn delete_goal(pool:&SqlitePool, successor_id: &str, predecessor_id: &str)->anyhow::Result<()>{
    let query = "DELETE FROM task_dependencies WHERE (successor_id=$1 AND predecessor_id=$2)";
    sqlx::query(query)
        .bind(successor_id)
        .bind(predecessor_id)
        .execute(pool)
        .await?;
    Ok(())
}



pub async fn get_task_dependencies(pool:&SqlitePool,goal_id:&str)->anyhow::Result<Vec<TaskDependency>>{
    let task_deps =
        sqlx::query_as::<_, TaskDependency>(r#"SELECT predecessor_id,successor_id,goal_id FROM task_dependencies 
                                                            WHERE ($1 IS NULL or group_id = $1)"#)
            .bind(goal_id)
            .fetch_all(pool)
            .await?;
    Ok(task_deps)
}




#[derive(Debug, FromRow)]
pub struct TaskFeedback {
    id: String,
    task_id: String,
    estimated_duration: u32,
    actual_duration: u32,
    user_sentiment: String,
    completion_quality: u8,
    created_at: String,
}