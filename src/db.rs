use std::{
    collections::HashMap,
    fs::File,
    path::Path,
};

use eyre::bail;
use serde::{
    Deserialize,
    Serialize,
};
use uuid::Uuid;

use crate::{
    operation::Operation,
    todo::Todo,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct Database {
    pub todos: HashMap<Uuid, Todo>,
    pub operations: Vec<Operation>,
}

pub fn load_database(path: impl AsRef<Path>) -> eyre::Result<Database> {
    let file = File::open(&path).or_else(|_| {
        let file = File::create(&path)?;
        let database = Database {
            todos: HashMap::new(),
            operations: Vec::new(),
        };
        serde_json::to_writer_pretty(&file, &database)?;
        File::open(&path)
    })?;
    let database = serde_json::from_reader(file)?;
    Ok(database)
}

pub fn save_database(database: Database, path: impl AsRef<Path>) -> eyre::Result<()> {
    let file = File::create(path)?;
    serde_json::to_writer_pretty(file, &database)?;
    Ok(())
}

pub fn create_pending_array(db: &Database) -> Vec<&Todo> {
    let mut array: Vec<_> = db
        .todos
        .values()
        .filter(|todo| !todo.is_finished())
        .collect();
    array.sort_by_key(|todo| todo.created_at);
    array
}

pub fn get_todo_id_by_index(db: &Database, index: usize) -> eyre::Result<Uuid> {
    let pending = create_pending_array(db);
    if index == 0 || index > pending.len() {
        bail!("id not found");
    }
    Ok(pending[index - 1].id)
}
