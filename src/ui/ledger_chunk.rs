use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans, Text},
    widgets::{
        Block, Borders, Cell, Clear, List, ListItem, ListState, Paragraph, Row, Table, TableState,
    },
    Frame, Terminal,
};

use crate::app::ledger_data::LedgerInputSelection;
use crate::app::{self, *};

use super::centered_rect;

pub fn ledger_chunk<B: Backend>(f: &mut Frame<B>, app: &mut App, working_chunk: &Vec<Rect>) {
    let rows = data_into_rows(app);
    let row = rows.iter().map(|item| {
        let height = item
            .iter()
            .map(|content| content.chars().filter(|c| *c == '\n').count())
            .max()
            .unwrap_or(0)
            + 1;
        let cells = item.iter().map(|c| Cell::from(c.clone()));
        Row::new(cells).height(height as u16).bottom_margin(0)
    });
    let normal_style = Style::default()
        .bg(Color::White)
        .add_modifier(Modifier::BOLD);
    let selected_style = Style::default().add_modifier(Modifier::REVERSED);
    let header_cells = ["Date", "Item: ", "Qty.", "Price"]
        .iter()
        .map(|h| Cell::from(*h).style(Style::default().fg(Color::Black)));
    let header = Row::new(header_cells)
        .style(normal_style)
        .height(1)
        .bottom_margin(0);
    let mut block = Block::default();
    match app.current_focus {
        Focus::LedgerFrame => {
            block = Block::default().title("Ledger").borders(Borders::ALL).border_style(Style::default().add_modifier(Modifier::BOLD));
        }
        _ => {
            block = Block::default().title("Ledger").borders(Borders::ALL);
        }
    }
    let budget_table = Table::new(row)
        .header(header)
        .block(block)
        .highlight_style(selected_style)
        .highlight_symbol(">> ")
        .widths(&[
            Constraint::Length(10),
            Constraint::Percentage(30),
            Constraint::Length(10),
            Constraint::Min(10),
        ]);
    f.render_stateful_widget(budget_table, working_chunk[1], &mut app.journal_vec[app.journal_state.selected().unwrap()].table_state);
}

fn data_into_rows(app: &mut App) -> Vec<Vec<String>> {
    let mut buffer = Vec::new();
    let journal_state_id = app.journal_state.selected().unwrap();
    if app.journal_vec.len() == 0 {
        buffer.push(vec!["".to_string(), "".to_string(), "".to_string(), "".to_string()])
    } else {
        for i in 0..app.journal_vec[journal_state_id].ledger_vec.len() {
            buffer.push(vec![
                app.journal_vec[journal_state_id].ledger_vec[i].date.to_string(),
                app.journal_vec[journal_state_id].ledger_vec[i].name.clone(),
                app.journal_vec[journal_state_id].ledger_vec[i]
                    .qty
                    .to_string(),
                app.journal_vec[journal_state_id].ledger_vec[i]
                    .price
                    .to_string(),
            ]);
        }
    }
    buffer
}

pub fn ledger_input<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let block = Block::default().borders(Borders::ALL);
    let date = Text::from(Span::raw(app.ledger_input.date.clone()));
    let name = Text::from(Span::raw(app.ledger_input.name.clone()));
    let qty = Text::from(Span::raw(app.ledger_input.qty.clone()));
    let price = Text::from(Span::raw(app.ledger_input.price.clone()));

    let input_date = Paragraph::new(date).block(
        Block::default()
            .style(Style::default().bg(Color::DarkGray))
            .borders(Borders::ALL)
            .title("Date: "),
    );
    let input_name = Paragraph::new(name).block(
        Block::default()
            .style(Style::default().bg(Color::DarkGray))
            .borders(Borders::ALL)
            .title("Name: "),
    );
    let input_qty = Paragraph::new(qty).block(
        Block::default()
            .style(Style::default().bg(Color::DarkGray))
            .borders(Borders::ALL)
            .title("Qty: "),
    );
    let input_price = Paragraph::new(price).block(
        Block::default()
            .style(Style::default().bg(Color::DarkGray))
            .borders(Borders::ALL)
            .title("Price: "),
    );

    let area = centered_rect(40, 30, f.size());
    f.render_widget(Clear, area);

    let chunk = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage(25),
                Constraint::Percentage(25),
                Constraint::Percentage(25),
                Constraint::Percentage(25)
            ]
            .as_ref(),
        )
        .split(area);

    match app.ledger_input.input_selection {
        LedgerInputSelection::Date => {
            f.set_cursor(
                chunk[0].x + app.ledger_input.date.len() as u16 + 1,
                chunk[0].y + 1,
            );
        }
        LedgerInputSelection::Name => {
            f.set_cursor(
                chunk[1].x + app.ledger_input.name.len() as u16 + 1,
                chunk[1].y + 1,
            );
        }
        LedgerInputSelection::Qty => {
            f.set_cursor(
                chunk[2].x + app.ledger_input.qty.len() as u16 + 1,
                chunk[2].y + 1,
            );
        }
        LedgerInputSelection::Price => {
            f.set_cursor(
                chunk[3].x + app.ledger_input.price.len() as u16 + 1,
                chunk[3].y + 1,
            );
        }
    }
    f.render_widget(input_date, chunk[0]);
    f.render_widget(input_name, chunk[1]);
    f.render_widget(input_qty, chunk[2]);
    f.render_widget(input_price, chunk[3]);
}
