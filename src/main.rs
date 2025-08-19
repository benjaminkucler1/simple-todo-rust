use console::style;
use crossterm::{
    cursor::MoveTo,
    execute,
    terminal::{Clear, ClearType},
};
use serde::{Deserialize, Serialize};
use std::io::{self, stdout};
use std::fmt;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Todo {
    id: u64,
    title: String,
    completed: bool,
}

struct AppState {
    next_id: u64,
    todos: Vec<Todo>,
}

impl AppState {
    pub fn new(next_id: u64) -> Self {
        AppState {
            next_id,
            todos: Vec::new(),
        }
    }
    pub fn todos(&self) -> &[Todo] {
        &self.todos
    }
    pub fn new_todo(&mut self, title: &str) -> u64 {
        let id = self.next_id;
        self.next_id += 1;

        self.todos.push(Todo::new(id, String::from(title)));
        id
    }
    pub fn edit_todo(&mut self, id: u64, new_title: &str) -> Option<&Todo> {
        if let Some(todo) = self.todos.iter_mut().find(|t| t.id == id) {
            todo.edit(new_title);
            Some(todo)
        } else {
            None
        }
    }
    pub fn delete_todo(&mut self, id: u64) -> Option<Todo> {
        if let Some(pos) = self.todos.iter().position(|t| t.id == id) {
            Some(self.todos.remove(pos))
        } else {
            None
        }

        //.retain(|t| t.id != id)
    }
    pub fn complete_todo(&mut self, id: u64) -> Option<&Todo> {
        if let Some(todo) = self.todos.iter_mut().find(|t| t.id == id) {
            todo.complete();
            Some(todo)
        } else {
            None
        }
    }
}

impl Todo {
    pub fn new(id: u64, title: String) -> Self {
        Todo {
            id,
            title,
            completed: false,
        }
    }

    pub fn complete(&mut self) {
        self.completed = true;
    }

    pub fn print(&self) {
        println!(
            "{} | {} | {}",
            self.id,
            self.title,
            if self.completed {
                style("Done").green()
            } else {
                style("To do").red()
            }
        );
    }
    pub fn edit(&mut self, new_title: &str) {
        self.title = String::from(new_title);
    }
}

impl fmt::Display for Todo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} | {} | {}",
            self.id,
            self.title,
            if self.completed {
                style("Done").green().to_string()
            } else {
                style("To do").red().to_string()
            }
        )
    }
}
fn edit_todo(app: &mut AppState, id: u64) -> Option<&Todo> {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("edit_todo: Failed to read the line");

    app.edit_todo(id, input.trim())
}

fn print_help() {
    println!("Available commands:");
    println!("  add             - Add a new todo");
    println!("  list            - List all todos");
    println!("  edit            - Edit todo");
    println!("  complete <id>   - Mark a todo as complete");
    println!("  help            - Show this help message");
    println!("  quit            - quit");
}

fn add_todo(app: &mut AppState) {
    let mut counter: u32 = 0;
    let mut input = String::new();
    loop {
        input.clear();
        println!("Enter a new todo item (or press Enter to finish):");
        io::stdin()
            .read_line(&mut input)
            .expect("add_todo: Failed to read line");

        if input.trim().is_empty() {
            break;
        }
        let _ = app.new_todo(input.trim());
        counter += 1;
        println!("{}", style("Added").green())
    }
    println!("Added {} todo items.", counter);
}

fn list_todos(todos: &[Todo]) {
    println!("----->Todos<-----");
    println!("Count: {}", todos.len());
    for todo in todos.iter() {
        todo.print();
    }
    println!("-----------------");
}

fn clear_screen() {
    //print!("\x1b[2J\x1b[H")
    execute!(stdout(), Clear(ClearType::All), MoveTo(0, 0)).unwrap();
}

fn init() -> AppState {
    AppState::new(0)
}

fn main() {
    let mut app: AppState = init();

    let mut input = String::new();
    print_help();
    loop {
        input.clear();

        println!("Enter a command: ");
        io::stdin()
            .read_line(&mut input)
            .expect("main: Failed to read line");

        match input.trim() {
            "add" | "a" => {
                let _ = add_todo(&mut app);
            }
            "list" | "ls" => list_todos(app.todos()),
            "complete" => println!("Please provide an ID (e.g., complete 3)"),
            s if s.starts_with("complete ") => {
                if let Some(id_str) = s.strip_prefix("complete ") {
                    if let Ok(id) = id_str.parse::<u64>() {
                        match app.complete_todo(id) {
                            Some(todo) => todo.print(),
                            None => println!("Todo not found!"),
                        }
                    }
                }
            }
            "c" => println!("Please provide an ID (e.g., c 3)"),
            s if s.starts_with("c ") => {
                if let Some(id_str) = s.strip_prefix("c ") {
                    if let Ok(id) = id_str.parse::<u64>() {
                        match app.complete_todo(id) {
                            Some(todo) => todo.print(),
                            None => println!("Todo not found!"),
                        }
                    }
                }
            }
            "edit" | "e" => println!("Please provide an ID (e.g. edit | e 4"),
            s if s.starts_with("edit ") => {
                if let Some(id_str) = s.strip_prefix("edit ") {
                    if let Ok(id) = id_str.parse::<u64>() {
                        match edit_todo(&mut app, id) {
                            Some(todo) => todo.print(),
                            None => println!("Todo not found!"),
                        }
                    }
                }
            }
            s if s.starts_with("e ") => {
                if let Some(id_str) = s.strip_prefix("e ") {
                    if let Ok(id) = id_str.parse::<u64>() {
                        match edit_todo(&mut app, id) {
                            Some(todo) => todo.print(),
                            None => println!("Todo not found!"),
                        }
                    }
                }
            }
            "delete" | "d" => println!("Please provide an ID (e.g. delete | d 4"),
            s if s.starts_with("delete ") => {
                if let Some(id_str) = s.strip_prefix("delete ") {
                    if let Ok(id) = id_str.parse::<u64>() {
                        match app.delete_todo(id) {
                            Some(todo) => todo.print(),
                            None => println!("Todo not found!"),
                        }
                    }
                }
            }
            s if s.starts_with("d ") => {
                if let Some(id_str) = s.strip_prefix("d ") {
                    if let Ok(id) = id_str.parse::<u64>() {
                        match app.delete_todo(id) {
                            Some(todo) => todo.print(),
                            None => println!("Todo not found!"),
                        }
                    }
                }
            }
            "quit" | "q" => break,
            "clear" | "cls" => clear_screen(),
            "help" | "h" | _ => print_help(),
        }
    }
}
