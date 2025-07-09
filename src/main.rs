use chrono::DateTime;
use chrono::Local;
use clap::Parser;
use clap::Subcommand;
use dirs::home_dir;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Write;
use std::path::PathBuf;

mod app;
mod tui;
mod ui;

const TODO_FILE_NAME: &str = ".todo";
const DATE_TIME_FORMAT: &str = "%m-%d %H:%M";

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Add a todo list item
    Add { description: Vec<String> },
    /// List all todo list items
    #[command(alias = "ls")]
    List,
    /// Remove a todo list item
    Remove { index: usize },
    /// Enter interactive move
    #[command(alias = "i")]
    Interactive,
}

struct TodoItem {
    timestamp: DateTime<Local>,
    description: String,
}

impl TodoItem {
    fn from_line(line: &str) -> Option<TodoItem> {
        let parts: Vec<&str> = line.splitn(2, '\t').collect();
        if parts.len() != 2 {
            return None;
        }
        let timestamp = DateTime::parse_from_rfc3339(parts[0]).ok()?;
        Some(TodoItem {
            timestamp: DateTime::from(timestamp),
            description: parts[1].to_string(),
        })
    }

    fn to_line(&self) -> String {
        format!("{}\t{}", self.timestamp.to_rfc3339(), self.description)
    }
}

fn get_todo_path() -> PathBuf {
    home_dir().unwrap().join(TODO_FILE_NAME)
}

fn read_todos() -> Vec<TodoItem> {
    let path = get_todo_path();
    if !path.exists() {
        return vec![];
    }

    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    reader
        .lines()
        .filter_map(|line| line.ok().and_then(|l| TodoItem::from_line(&l)))
        .collect()
}

fn write_todos(todos: &[TodoItem]) {
    let path = get_todo_path();
    let mut file = File::create(path).unwrap();
    for todo in todos {
        writeln!(file, "{}", todo.to_line()).unwrap();
    }
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Add { description } => {
            let desc = description.join(" ");
            let new_item = TodoItem {
                timestamp: Local::now(),
                description: desc.clone(),
            };

            let mut file = OpenOptions::new()
                .create(true)
                .append(true)
                .open(get_todo_path())
                .unwrap();
            writeln!(file, "{}", new_item.to_line()).unwrap();
            println!(
                "Added: {}\n{}",
                desc,
                new_item.timestamp.format(DATE_TIME_FORMAT)
            );
        }
        Commands::List => {
            let todos = read_todos();
            if todos.is_empty() {
                println!("No todos");
            } else {
                for (i, todo) in todos.iter().enumerate() {
                    println!(
                        "{}. [{}] {}",
                        i + 1,
                        todo.timestamp.format(DATE_TIME_FORMAT),
                        todo.description
                    );
                }
            }
        }
        Commands::Remove { index } => {
            let mut todos = read_todos();
            if index == 0 || index > todos.len() {
                println!("Invalid todo index: {}", index);
            } else {
                let removed = todos.remove(index - 1);
                write_todos(&todos);
                println!("Removed: {}", removed.description);
            }
        }
        Commands::Interactive => {
            let _ = tui::interactive_mode();
        }
    }
}
