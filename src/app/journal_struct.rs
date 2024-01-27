use chrono::NaiveDate;
use tui::widgets::TableState;

use crate::app::ledger_data::LedgerData;

#[derive(Clone)]
pub struct JournalData {
    pub name: String,
    pub journal_vec: Vec<JournalData>,
    pub ledger_vec: Vec<LedgerData>,
    pub table_state: TableState,
}

impl JournalData {
    pub fn new(name: String) -> JournalData {
        let mut buffer = JournalData {
            name: name,
            journal_vec: Vec::new(),
            ledger_vec: Vec::new(),
            table_state: TableState::default(),
        };
        buffer.table_state.select(Some(0));
        buffer
    }

    pub fn new_ledger_entry(&mut self, date: String, name: String, qty: String, price: String) {
        self.ledger_vec
            .push(LedgerData::new(date, name, qty, price));
        self.sort_date();
    }

    pub fn previous(&mut self) {
        if self.ledger_vec.len() != 0 {
            let i = match self.table_state.selected() {
                Some(i) => {
                    if i == 0 {
                        self.table_state.select(Some(i))
                    } else {
                        self.table_state.select(Some(i - 1))
                    }
                }
                None => (),
            };
        }
    }

    pub fn next(&mut self) {
        if self.ledger_vec.len() != 0 {
            let i = match self.table_state.selected() {
                Some(i) => {
                    if i == self.ledger_vec.len() - 1 {
                        self.table_state.select(Some(self.ledger_vec.len() - 1))
                    } else {
                        self.table_state.select(Some(i + 1))
                    }
                }
                None => (),
            };
        }
    }

    pub fn delete(&mut self) {
        let mut buffer: Vec<LedgerData> = Vec::new();
        for i in 0..self.ledger_vec.len() {
            if i != self.table_state.selected().unwrap() {
                buffer.push(self.ledger_vec[i].clone())
            }
        }
        self.ledger_vec = buffer;
        if self.table_state.selected().unwrap() > 0 {
            self.table_state
                .select(Some(self.table_state.selected().unwrap() - 1));
        }
    }

    pub fn sort_date(&mut self) {
        loop {
            if sort_naive_date_struct(self) == false {
                break;
            }
        }
    }
}

pub struct JournalInput {
    pub input: String,
}

impl JournalInput {
    pub fn new() -> JournalInput {
        JournalInput {
            input: String::new(),
        }
    }
}

fn sort_naive_date_struct(ledger: &mut JournalData) -> bool {
    let mut continue_loop = false;
    let mut continue_loop_buffer = false;
    for i in 0..ledger.ledger_vec.len() - 1 {
        let date_str_1 = ledger.ledger_vec[i].date.clone();
        let date_str_2 = ledger.ledger_vec[i + 1].date.clone();
        let date_struct_1: NaiveDate = NaiveDate::parse_from_str(&date_str_1, "%Y-%m-%d").unwrap();
        let date_struct_2: NaiveDate = NaiveDate::parse_from_str(&date_str_2, "%Y-%m-%d").unwrap();

        if date_struct_1 <= date_struct_2 {
            continue_loop = false;
        } else {
            let buffer_1 = ledger.ledger_vec[i].clone();
            let buffer_2 = ledger.ledger_vec[i + 1].clone();

            ledger.ledger_vec[i] = buffer_2;
            ledger.ledger_vec[i + 1] = buffer_1;
            continue_loop = true;
        }
        if continue_loop_buffer == false {
            continue_loop_buffer = continue_loop;
        }
    }
    continue_loop_buffer
}
