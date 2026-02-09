use serde::{
    Deserialize,
    Serialize,
};
use uuid::Uuid;

use crate::db::Database;

#[derive(Debug, Serialize, Deserialize)]
pub enum Operation {
    Add { uuid: Uuid },
    Edit { uuid: Uuid, old_description: String },
    Finish { uuid: Uuid },
}

pub fn undo_operation(db: &mut Database, operation: Operation) {
    match operation {
        Operation::Add { uuid } => undo_add(db, uuid),
        Operation::Edit {
            uuid,
            old_description,
        } => undo_edit(db, uuid, old_description),
        Operation::Finish { uuid } => undo_finish(db, uuid),
    }
}

fn undo_add(db: &mut Database, uuid: Uuid) {
    db.todos.remove(&uuid);
}

fn undo_edit(db: &mut Database, uuid: Uuid, old_description: String) {
    let todo = db.todos.get_mut(&uuid).unwrap();
    todo.description = old_description;
}

fn undo_finish(db: &mut Database, uuid: Uuid) {
    let todo = db.todos.get_mut(&uuid).unwrap();
    todo.finished_at = None;
}
