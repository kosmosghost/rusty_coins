use tui::{backend::Backend, layout::{Rect, Alignment}, Frame, widgets::{Paragraph, Block}};

use crate::app::App;



pub fn title_chunk<B: Backend>(f: &mut Frame<B>, app: &mut App, main_chunk: &Vec<Rect>) {
    let title = Paragraph::new("Rusty Coins v_0.0.1").block(Block::default()).alignment(Alignment::Center);
    f.render_widget(title, main_chunk[0]);
}
