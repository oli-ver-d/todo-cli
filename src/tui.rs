use crate::app::App;
use crate::app::Mode;
use crate::read_todos;
use crate::ui::ui;
use crate::write_todos;
use anyhow::Result;
use crossterm::event;
use crossterm::event::DisableMouseCapture;
use crossterm::event::EnableMouseCapture;
use crossterm::event::Event;
use crossterm::event::KeyCode;
use crossterm::execute;
use crossterm::terminal::EnterAlternateScreen;
use crossterm::terminal::LeaveAlternateScreen;
use crossterm::terminal::disable_raw_mode;
use crossterm::terminal::enable_raw_mode;
use ratatui::Terminal;
use ratatui::prelude::Backend;
use ratatui::prelude::CrosstermBackend;
use std::io;

pub fn interactive_mode() -> Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // run the fucking application dumbass
    let todos = read_todos();
    let mut app = App::new(todos);
    app.list_state.select(Some(0));
    run_app(&mut terminal, &mut app)?;

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> Result<()> {
    loop {
        terminal.draw(|f| ui(f, app))?;

        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Release {
                continue;
            }

            match app.mode {
                Mode::Normal => match key.code {
                    KeyCode::Char('i') => {
                        app.mode = Mode::Insert;
                    }
                    KeyCode::Char('o') => {
                        app.mode = Mode::New;
                        app.insert_new();
                        app.list_state.select_next();
                    }
                    KeyCode::Char(':') => {
                        app.mode = Mode::Command;
                    }
                    KeyCode::Char('j') => {
                        app.list_state.select_next();
                    }
                    KeyCode::Char('k') => {
                        app.list_state.select_previous();
                    }
                    KeyCode::Char('g') => {
                        app.list_state.select_first();
                    }
                    KeyCode::Char('G') => {
                        app.list_state.select_last();
                    }
                    KeyCode::Enter | KeyCode::Char(' ') => {
                        // Todo mark as completed
                        app.toggle_status();
                    }
                    _ => {}
                },
                Mode::Command => match key.code {
                    KeyCode::Esc => {
                        app.mode = Mode::Normal;
                        app.command = String::new();
                    }
                    KeyCode::Backspace => {
                        app.command.pop();
                    }
                    KeyCode::Char(value) => {
                        app.command.push(value);
                    }
                    KeyCode::Enter => {
                        // Execute the command!
                        if app.command == "q" {
                            break;
                        }
                        if app.command == "wq" {
                            write_todos(&app.todos);
                            break;
                        }
                        if app.command == "w" {
                            write_todos(&app.todos);
                        }
                        app.command = String::new();
                        app.mode = Mode::Normal;
                    }
                    _ => {}
                },
                Mode::New | Mode::Insert => match key.code {
                    KeyCode::Esc => {
                        if app.mode == Mode::New {
                            if let Some(i) = app.list_state.selected() {
                                app.todos.remove(i);
                            }
                            app.mode = Mode::Normal;
                        } else {
                            app.mode = Mode::Normal;
                        }
                    }
                    KeyCode::Char(value) => {
                        if let Some(i) = app.list_state.selected() {
                            app.todos[i].description.push(value);
                        }
                    }
                    KeyCode::Backspace => {
                        if let Some(i) = app.list_state.selected() {
                            app.todos[i].description.pop();
                        }
                    }
                    KeyCode::Enter => {
                        app.mode = Mode::Normal;
                    }
                    _ => {}
                },
            }
        }
    }
    Ok(())
}
