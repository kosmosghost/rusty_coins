use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    widgets::{Block, Borders, Clear, ListState, Paragraph},
    Frame, Terminal,
};

mod title_chunk;
mod journal_chunk;
mod ledger_chunk;
mod help;
use crate::app::{App, Focus};

pub fn start_ui<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let main_chunk = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Max(5), Constraint::Percentage(85), Constraint::Percentage(10)].as_ref())
        .split(f.size());

    title_chunk::title_chunk(f, app, &main_chunk);

    let working_chunk = Layout::default().direction(Direction::Horizontal).constraints([Constraint::Percentage(25), Constraint::Percentage(75)].as_ref()).split(main_chunk[1]);
    journal_chunk::journal_chunk(f, app, &working_chunk);

    match app.journal_vec.len() {
        0 => {
            help::ledger_chunk(f, app, &working_chunk);
        }
        _ => {
            ledger_chunk::ledger_chunk(f, app, &working_chunk);
        }
    }
    
    //Render Menus:
    match app.current_focus {
        Focus::JournalInput => {
            journal_chunk::journal_input(f, app);
        }
        Focus::LedgerInput => {
            ledger_chunk::ledger_input(f, app);
        }
        _ => ()
    }
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage((100 - percent_y) / 2),
                Constraint::Percentage(percent_y),
                Constraint::Percentage((100 - percent_y) / 2),
            ]
            .as_ref(),
        )
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage((100 - percent_x) / 2),
                Constraint::Percentage(percent_x),
                Constraint::Percentage((100 - percent_x) / 2),
            ]
            .as_ref(),
        )
        .split(popup_layout[1])[1]
}
