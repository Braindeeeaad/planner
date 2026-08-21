use serde::{Deserialize, Serialize};
use sqlx::{Sqlite, Transaction};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Op {
    AddNode {
        id: String,
        label: String,
        x: Option<f64>,
        y: Option<f64>,
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
        x: f64,
        y: f64,
    },
    RenameNode {
        id: String,
        label: String,
    },
    Batch {
        ops: Vec<Op>,
    },
}


async fn apply_op(tx: &mut Transaction<'_, Sqlite>, goal_id: &str, op: &Op) -> anyhow::Result<()> {
    match op {
        Op::AddNode { id, label, x, y } => {
            sqlx::query("INSERT INTO nodes (id, goal_id, label, x, y) VALUES (?, ?, ?, ?, ?)")
                .bind(id).bind(goal_id).bind(label).bind(x).bind(y)
                .execute(&mut **tx).await?;
        }
        Op::AddEdge { predecessor_id, successor_id } => {
            // run cycle check against current tx state before inserting
            //if would_create_cycle(tx, goal_id, predecessor_id, successor_id).await? {
            //    return Err(DagError::EdgeError { message: "would create a cycle".into() }.into());
           // }
            sqlx::query("INSERT INTO task_dependencies (predecessor_id, successor_id, goal_id) VALUES (?, ?, ?)")
                .bind(predecessor_id).bind(successor_id).bind(goal_id)
                .execute(&mut **tx).await?;
        }
        Op::MoveNode { id, x, y } => {
            sqlx::query("UPDATE nodes SET x = ?, y = ? WHERE id = ? AND goal_id = ?")
                .bind(x).bind(y).bind(id).bind(goal_id)
                .execute(&mut **tx).await?;
        }
        Op::Batch { ops } => {
            for sub_op in ops {
                Box::pin(apply_op(tx, goal_id, sub_op)).await?; // recursion needs boxing (async fn)
            }
        }
        // RemoveNode, RemoveEdge, RenameNode similarly...
        _ => todo!(),
    }
    Ok(())
}


