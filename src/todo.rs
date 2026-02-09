use chrono::{
    DateTime,
    Utc,
};
use color_eyre::owo_colors::OwoColorize;
use serde::{
    Deserialize,
    Serialize,
};
use uuid::Uuid;

use crate::{
    db::Database,
    operation::Operation,
    utils::format_duration,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct Todo {
    pub id: Uuid,
    pub description: String,
    pub created_at: DateTime<Utc>,
    pub finished_at: Option<DateTime<Utc>>,
}

impl Todo {
    pub fn is_finished(&self) -> bool {
        self.finished_at.is_some()
    }
}

pub fn add_todo(db: &mut Database, description: String) {
    let id = Uuid::new_v4();
    let todo = Todo {
        id: id.clone(),
        description,
        created_at: Utc::now(),
        finished_at: None,
    };
    db.todos.insert(todo.id, todo);
    db.operations.push(Operation::Add { uuid: id });
}

pub fn edit_todo(db: &mut Database, id: Uuid, new_description: String) {
    let todo = db.todos.get_mut(&id).unwrap();
    let old_description = todo.description.clone();
    todo.description = new_description;
    db.operations.push(Operation::Edit {
        uuid: id,
        old_description,
    });
}

pub fn finish_todo(db: &mut Database, id: Uuid) {
    let todo = db.todos.get_mut(&id).unwrap();
    todo.finished_at = Some(Utc::now());
    println!(
        "     🗹 {} {}",
        todo.description.bold().strikethrough(),
        format_duration(Utc::now() - todo.created_at).dimmed()
    );
    db.operations.push(Operation::Finish { uuid: id });
}
