use crate::db::connection::{Saved, New, Changed};
use sqlx::sqlite::SqlitePool;
use sqlx::{FromRow, Row, sqlite::SqliteRow};
use std::marker::PhantomData;
use uuid::Uuid;
use crate::core::dag::Action;
use std::any::TypeId;


#[derive(Clone)]
pub struct Task<State = New> {
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
    _state: PhantomData<State>,
}


/*

    Task trait implementation


*/
impl <'r, State> FromRow<'r, SqliteRow> for Task<State> {
    fn from_row(row: &'r SqliteRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            id: row.try_get("id")?,
            goal_id: row.try_get("goal_id")?,
            event_context_id: row.try_get("event_context_id")?,
            title: row.try_get("title")?,
            task_type: row.try_get("task_type")?,
            base_duration: row.try_get("base_duration")?,
            schedule_start: row.try_get("schedule_start")?,
            schedule_end: row.try_get("schedule_end")?,
            urgency_score: row.try_get("urgency_score")?,
            importance_score: row.try_get("importance_score")?,
            priority_weight: row.try_get("priority_weight")?,
            status: row.try_get("status")?,
            _state: PhantomData,
        })
    }
}

impl<State:'static> Action for Task<State>{
    fn get_uuid(&self)->&str {
        &self.id
    }
    fn upload(&self)->Result<(),sqlx::Error> {
        if TypeId::of::<State>() == TypeId::of::<New>(){

        }
        Ok(())
    }
}


impl Task<New> {
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
            _state: PhantomData,
        }
    }
}

pub async fn create_task(task: Task<New>, pool: &SqlitePool) -> anyhow::Result<Task<Saved>> {
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

    Ok(Task {
        id: task.id,
        goal_id: task.goal_id,
        event_context_id: task.event_context_id,
        title: task.title,
        task_type: task.task_type,
        base_duration: task.base_duration,
        schedule_start: task.schedule_start,
        schedule_end: task.schedule_end,
        urgency_score: task.urgency_score,
        importance_score: task.importance_score,
        priority_weight: task.priority_weight,
        status: task.status,
        _state: PhantomData,
    })
}

pub async fn get_task_by_goal(
    goal_id: &str,
    pool: &SqlitePool,
) -> anyhow::Result<Vec<Task<Saved>>> {
    let tasks = sqlx::query_as::<_, Task<Saved>>(
        r#"SELECT id,goal_id,event_context_id,title,task_type,base_duration,schedule_start,schedule_end,urgency_score,importance_score,priority_weight,status FROM tasks WHERE goal_id = $1"#,
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
}

pub async fn upload_task_dependency(
    predecessor_id: &str,
    successor_id: &str,
    pool: &SqlitePool,
) -> anyhow::Result<()> {
    let query = "INSERT INTO task_dependencies (predecessor_id,successor_id) VALUES ($1,$2)";
    sqlx::query(query)
        .bind(predecessor_id)
        .bind(successor_id)
        .execute(pool)
        .await?;
    Ok(())
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