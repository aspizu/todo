use std::path::PathBuf;

use chrono::Utc;
use clap::{
    Parser,
    builder::{
        Styles,
        styling::{
            AnsiColor,
            Effects,
        },
    },
};
use color_eyre::owo_colors::OwoColorize;
use eyre::bail;

use crate::{
    db::{
        create_pending_array,
        load_database,
        save_database,
    },
    operation::undo_operation,
    todo::{
        add_todo,
        edit_todo,
        finish_todo,
    },
    utils::format_duration,
};

pub const CLAP_STYLES: Styles = Styles::styled()
    .header(AnsiColor::Green.on_default().effects(Effects::BOLD))
    .usage(AnsiColor::Green.on_default().effects(Effects::BOLD))
    .literal(AnsiColor::Cyan.on_default().effects(Effects::BOLD))
    .placeholder(AnsiColor::Cyan.on_default());

#[derive(Parser)]
#[command(styles=CLAP_STYLES)]
#[command(
    about = "Run with no arguments to list pending todos, or with a description to add a new todo item."
)]
struct Cli {
    /// Mark a todo item as finished.
    #[arg(short, long)]
    finish: Option<usize>,
    /// Edit the description of a todo item.
    #[arg(short, long)]
    edit: Option<usize>,
    /// Undo the last operation.
    #[arg(short, long)]
    undo: bool,
    /// Path to the todo list file.
    #[arg(long)]
    list: Option<PathBuf>,
    /// Description of the new todo item.
    #[arg()]
    args: Vec<String>,
}

pub fn run() -> eyre::Result<()> {
    let cli = Cli::parse();
    let dbpath = cli.list.unwrap_or(".todos".into());
    let mut db = load_database(&dbpath)?;
    if let Some(id) = cli.finish {
        let pending = create_pending_array(&db);
        if id == 0 || id > pending.len() {
            bail!("id not found");
        }
        let id = pending[id - 1].id;
        finish_todo(&mut db, id);
    } else if let Some(id) = cli.edit {
        let new_description = cli.args.join(" ");
        let new_description = new_description.trim();
        if new_description.is_empty() {
            bail!("missing new description");
        }
        let pending = create_pending_array(&db);
        if id == 0 || id > pending.len() {
            bail!("id not found");
        }
        let id = pending[id - 1].id;
        edit_todo(&mut db, id, new_description.to_owned());
    } else if cli.undo {
        let Some(operation) = db.operations.pop() else {
            bail!("no operations to undo");
        };
        undo_operation(&mut db, operation);
    } else {
        let description = cli.args.join(" ");
        let description = description.trim();
        if description.is_empty() {
            let now = Utc::now();
            let pending = create_pending_array(&db);
            for (i, todo) in pending.iter().enumerate() {
                println!(
                    "{:>4} ☐ {} {}",
                    i + 1,
                    todo.description.bold(),
                    format_duration(now - todo.created_at).dimmed()
                );
            }
        } else {
            add_todo(&mut db, description.to_owned());
        }
    }
    save_database(db, dbpath)?;
    Ok(())
}
