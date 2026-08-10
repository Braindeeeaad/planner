use crate::db::connection::{Saved, New};
use sqlx::sqlite::SqlitePool;
use sqlx::{FromRow, Row, sqlite::SqliteRow};
use std::marker::PhantomData;
use uuid::Uuid;

pub struct Goal<State = New> {
    id: String,
    title: String,
    target_date: String,
    status: String,
    _state: PhantomData<State>,
}

impl<'r, State> FromRow<'r, SqliteRow> for Goal<State> {
    fn from_row(row: &'r SqliteRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            id: row.try_get("id")?,
            title: row.try_get("title")?,
            target_date: row.try_get("target_date")?,
            status: row.try_get("status")?,
            _state: PhantomData,
        })
    }
}

impl Goal<New> {
    pub fn new(title: String, target_date: String, status: String) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            title,
            target_date,
            status,
            _state: PhantomData,
        }
    }
}

pub async fn upload_goal(goal: Goal<New>, pool: &SqlitePool) -> anyhow::Result<Goal<Saved>> {
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
        _state: PhantomData,
    })
}

pub async fn get_goals(pool: &SqlitePool) -> anyhow::Result<Vec<Goal<Saved>>> {
    let goals =
        sqlx::query_as::<_, Goal<Saved>>(r#"SELECT id,title,target_date,status FROM goals"#)
            .fetch_all(pool)
            .await?;
    Ok(goals)
}