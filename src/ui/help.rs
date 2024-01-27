use tui::{
    backend::Backend,
    layout::{Rect, Alignment},
    style::{Style, Modifier},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::app::{App, Focus};

pub fn ledger_chunk<B: Backend>(f: &mut Frame<B>, app: &mut App, working_chunk: &Vec<Rect>) {
    let help_text: &str = "Welcome to Rusty Coins.\n
    Press \'n\' to create a new Journal Entry";

    let mut block = Block::default();
    match app.current_focus {
        Focus::LedgerFrame => {
            block = Block::default().title("Empty Ledger").borders(Borders::ALL).border_style(Style::default().add_modifier(Modifier::BOLD));
        }
        _ => {
            block = Block::default().title("Empty Ledger").borders(Borders::ALL);

        }
    }
    let paragraph = Paragraph::new(help_text).block(block).alignment(Alignment::Center);

    f.render_widget(paragraph, working_chunk[1]);
}
