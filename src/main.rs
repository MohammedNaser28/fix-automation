use ratatui::{backend::CrosstermBackend, Terminal};
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io;
use crate::app::ConfirmFocus;
// mod app;
mod screens;
mod sys;
mod app;
mod ui;
// mod ui;

use crate::app::{App, CurrentScreen};

fn main() -> Result<(), io::Error> {
    // Terminal Setup
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Application State
    let mut app = App::new();

    // Main Loop
    loop {
        terminal.draw(|f| screens::render(f, &mut app))?;

        if let Event::Key(key) = event::read()? {
            if key.code == KeyCode::Char('q') { break; }

            // Delegate input handling to screens
            handle_input(&mut app, key.code);
        }

        if app.should_quit { break; }
    }

    // Restore Terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    Ok(())
}

fn handle_input(app: &mut App, code: KeyCode) {
    match app.current_screen {
        CurrentScreen::Welcome => {
            if code == KeyCode::Enter {
                app.disks = sys::blkdev::get_disks();
                app.current_screen = CurrentScreen::SelectRoot;
                app.table_state.select(Some(0)); // Start at first disk
            }
        }
        CurrentScreen::SelectRoot => {
            match code {
                KeyCode::Up => app.select_previous(),
                KeyCode::Down => app.select_next(),
                KeyCode::Enter => {
                    if let Some(i) = app.table_state.selected() {
                        app.selected_root = Some(app.disks[i].clone());
                        app.current_screen = CurrentScreen::SelectEfi;
                        app.table_state.select(Some(0)); // Reset for next screen
                    }
                }
                _ => {}
            }
        }
        CurrentScreen::SelectEfi => {
            match code {
                KeyCode::Up => app.select_previous(),
                KeyCode::Down => app.select_next(),
                KeyCode::Enter => {
                    if let Some(i) = app.table_state.selected() {
                        app.selected_efi = Some(app.disks[i].clone());
                        app.current_screen = CurrentScreen::Confirm;
                    }
                }
                KeyCode::Esc => app.current_screen = CurrentScreen::SelectRoot,
                _ => {}
            }
        }
        CurrentScreen::Confirm => {
            match code {
                KeyCode::Left | KeyCode::Right | KeyCode::Tab => {
                    app.toggle_confirm_buttons();
                }

                KeyCode::Enter => {
                    match app.confirm_focus {
                        ConfirmFocus::Confirm => {
                            app.current_screen = CurrentScreen::ActionMenu;
                        }
                        ConfirmFocus::Back => {
                            app.current_screen = CurrentScreen::SelectEfi;
                        }
                    }
                }

                KeyCode::Esc => {
                    app.current_screen = CurrentScreen::SelectEfi;
                }

                _ => {}
            }
        }
        CurrentScreen::ActionMenu => {}
        CurrentScreen::ExecLog    => {}
    }
}