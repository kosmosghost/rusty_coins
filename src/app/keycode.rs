use crossterm::event::{self, Event, KeyCode};
use tui::{backend::Backend, Terminal};

use crate::app::*;

use super::ledger_data::LedgerInputSelection;

pub fn read_keys(app: &mut App) -> bool {
    let mut quit_app: bool = false;
    match app.current_focus {
        Focus::JournalFrame => {
            journal_frame_keycode(app, &mut quit_app);
        }
        Focus::LedgerFrame => ledger_frame_keycode(app, &mut quit_app),
        Focus::JournalInput => {
            journal_input_keycode(app);
        }
        Focus::LedgerInput => {
            ledger_input_keycode(app);
        }
        _ => {
            panic!("Cannot operate KeyCode!");
        }
    }
    if quit_app == true {
        return false;
    } else {
        return true;
    }
}

pub fn journal_frame_keycode(app: &mut App, quit_app: &mut bool) {
    if let Event::Key(key) = event::read().unwrap() {
        match key.code {
            KeyCode::Char('q') => *quit_app = true,
            KeyCode::Char('s') => app.export_data_to_json(),
            KeyCode::Char('o') => app.import_data_from_json(),
            KeyCode::Down => {
                app.next();
            }
            KeyCode::Up => {
                app.previous();
            }
            KeyCode::Char('n') => {
                app.previous_focus = app.current_focus;
                app.current_focus = Focus::JournalInput;
            }
            KeyCode::Tab => {
                app.current_focus = Focus::LedgerFrame;
                app.previous_focus = Focus::LedgerFrame;
            }
            _ => *quit_app = false,
        }
    } else {
        *quit_app = false;
    }
}

pub fn journal_input_keycode(app: &mut App) {
    if let Event::Key(key) = event::read().unwrap() {
        match key.code {
            KeyCode::Esc => {
                app.current_focus = app.previous_focus;
                app.journal_input.input.clear();
            }
            KeyCode::Char(c) => {
                app.journal_input.input.push(c);
            }
            KeyCode::Backspace => {
                app.journal_input.input.pop();
            }
            KeyCode::Enter => {
                app.new_journal_entry(app.journal_input.input.clone());
                app.journal_input.input.clear();
                app.current_focus = app.previous_focus;
            }

            _ => (),
        }
    }
}

pub fn ledger_frame_keycode(app: &mut App, quit_app: &mut bool) {
    if app.journal_vec.len() == 0 {
        if let Event::Key(key) = event::read().unwrap() {
            match key.code {
                KeyCode::Char('q') => *quit_app = true,
                KeyCode::Tab => {
                    app.current_focus = Focus::JournalFrame;
                    app.previous_focus = Focus::JournalFrame;
                }    
                _ => *quit_app = false,
            }
        } else {
            *quit_app = false;
        }

    }
    else {
        if let Event::Key(key) = event::read().unwrap() {
            match key.code {
                KeyCode::Char('q') => *quit_app = true,
                KeyCode::Char('n') => {
                    app.previous_focus = app.current_focus;
                    app.current_focus = Focus::LedgerInput;
                }
                KeyCode::Delete => {
                    app.journal_vec[app.journal_state.selected().unwrap()].delete();
                }
                KeyCode::Tab => {
                    app.current_focus = Focus::JournalFrame;
                    app.previous_focus = Focus::JournalFrame;
                }
                KeyCode::Down => {
                    app.journal_vec[app.journal_state.selected().unwrap()].next();
                }
                KeyCode::Up => {
                    app.journal_vec[app.journal_state.selected().unwrap()].previous();
                }

                _ => *quit_app = false,
            }
            
        } else {
            *quit_app = false;
        }
    }
}

pub fn ledger_input_keycode(app: &mut App) {
    if let Event::Key(key) = event::read().unwrap() {
        match key.code {
            KeyCode::Esc => {
                app.current_focus = app.previous_focus;
                app.ledger_input.clear();
            }
            KeyCode::Char(c) => {
                match app.ledger_input.input_selection {
                    LedgerInputSelection::Date => {
                        app.ledger_input.date.push(c);
                    }
                    LedgerInputSelection::Name => {
                        app.ledger_input.name.push(c);
                    }
                    LedgerInputSelection::Qty => {
                        app.ledger_input.qty.push(c);
                    }
                    LedgerInputSelection::Price => {
                        app.ledger_input.price.push(c);
                    }
                }
            }
            KeyCode::Backspace => {
                match app.ledger_input.input_selection {
                    LedgerInputSelection::Date => {
                        app.ledger_input.date.pop();
                    }
                    LedgerInputSelection::Name => {
                        app.ledger_input.name.pop();
                    }
                    LedgerInputSelection::Qty => {
                        app.ledger_input.qty.pop();
                    }
                    LedgerInputSelection::Price => {
                        app.ledger_input.price.pop();
                    }

            }
        }
            KeyCode::Enter => {
                match app.ledger_input.input_selection {
                    LedgerInputSelection::Date => {
                        app.ledger_input.input_selection = LedgerInputSelection::Name;
                    }
                    LedgerInputSelection::Name => {
                        app.ledger_input.input_selection = LedgerInputSelection::Qty;
                    }
                    LedgerInputSelection::Qty => {
                        app.ledger_input.input_selection = LedgerInputSelection::Price;
                    }
                    LedgerInputSelection::Price => {
                        let state_id = app.journal_state.selected().unwrap();
                        let (date, name, qty, price) = app.ledger_input.retrieve_input_items();
                        app.journal_vec[state_id].new_ledger_entry(date.clone(), name.clone(), qty.clone(), price.clone());
                        app.ledger_input.clear();
                        app.ledger_input.input_selection = LedgerInputSelection::Name;
                    }
                }
            }
            KeyCode::BackTab => {
                match app.ledger_input.input_selection {
                    LedgerInputSelection::Date => {
                        app.ledger_input.input_selection = LedgerInputSelection::Price;
                    }
                    LedgerInputSelection::Name => {
                        app.ledger_input.input_selection = LedgerInputSelection::Date;
                    }
                    LedgerInputSelection::Qty => {
                        app.ledger_input.input_selection = LedgerInputSelection::Name;
                    }
                    LedgerInputSelection::Price => {
                        app.ledger_input.input_selection = LedgerInputSelection::Qty;
                    }
                }
            }
            KeyCode::Tab => {
                match app.ledger_input.input_selection {
                    LedgerInputSelection::Date => {
                        app.ledger_input.input_selection = LedgerInputSelection::Name;
                    }
                    LedgerInputSelection::Name => {
                        app.ledger_input.input_selection = LedgerInputSelection::Qty;
                    }
                    LedgerInputSelection::Qty => {
                        app.ledger_input.input_selection = LedgerInputSelection::Price;
                    }
                    LedgerInputSelection::Price => {
                        app.ledger_input.input_selection = LedgerInputSelection::Date;
                    }
                }
            }

            _ => (),
        }
    }
}