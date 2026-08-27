use std::sync::Mutex;
use std::collections::HashMap;
use sqlx::{SqlitePool};


use crate::db::connection::{create_database, establish_connection};

use crate::core::dag::{Dag};
use crate::models::goal::{Goal,get_goals};


#[derive(Default)]
pub struct AppState{
    pub dag_map:HashMap<String,Dag>
}

pub async fn initilize_state(state:&mut AppState)->anyhow::Result<()>{
    let pool = establish_connection().await?;
    assert!(state.dag_map.is_empty());
    let goals=get_goals(&pool).await?;
    for goal in goals{
        state.dag_map.insert(String::from(goal.get_id()),Dag::create_with_goal(goal).await?);
    }
    Ok(())
}

