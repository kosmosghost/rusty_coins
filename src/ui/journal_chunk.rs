use tui::{
    backend::Backend,
    layout::{Rect, Layout, Constraint, Direction},
    style::{Color, Modifier, Style},
    text::{Span, Spans, Text},
    widgets::{Block, Borders, List, ListItem, Clear, Paragraph},
    Frame,
};

use super::centered_rect;
use crate::app::App;
use crate::app::Focus;

pub fn journal_chunk<B: Backend>(f: &mut Frame<B>, app: &mut App, working_chunk: &Vec<Rect>) {
    let items: Vec<_> = app
        .get_index_list()
        .iter()
        .map(|list| {
            ListItem::new(Spans::from(vec![Span::styled(
                list.clone(),
                Style::default(),
            )]))
        })
        .collect();

    let mut block = Block::default();
    match app.current_focus {
        Focus::JournalFrame => {
            block = Block::default().title("Journals").borders(Borders::ALL).border_style(Style::default().add_modifier(Modifier::BOLD));
        }
        _ => {
            block = Block::default().title("Journals").borders(Borders::ALL);
        }
    }
    let list = List::new(items).block(block).highlight_style(
        Style::default()
            .bg(Color::White)
            .fg(Color::Black)
            .add_modifier(Modifier::BOLD),
    );
    f.render_stateful_widget(list, working_chunk[0], &mut app.journal_state);
}

pub fn journal_input<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let block = Block::default().borders(Borders::ALL);
    let text = Text::from(Span::raw(app.journal_input.input.clone()));
    let input_name = Paragraph::new(text).block(Block::default().borders(Borders::ALL).title("New Journal Name: ").style(Style::default().bg(Color::DarkGray)));
    

    let area = centered_rect(40, 15, f.size());
    f.render_widget(Clear, area);

    let chunk = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(50),
            Constraint::Percentage(50),
        ].as_ref())
        .split(area);

        f.set_cursor(chunk[0].x + app.journal_input.input.len() as u16 + 1, chunk[0].y + 1);
        f.render_widget(input_name, chunk[0]);
}
