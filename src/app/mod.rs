use json::{object, JsonValue};

use std::io::{self, Read, Write};

use tui::{backend::Backend, widgets::ListState, Terminal};

mod journal_struct;
mod keycode;
mod reports_struct;
pub(crate) mod ledger_data;

use crate::ui;

use self::{
    journal_struct::{JournalData, JournalInput},
    ledger_data::{LedgerData, LedgerInput},
};

#[derive(Clone, Copy)]
pub enum Focus {
    JournalFrame,
    JournalInput,
    LedgerFrame,
    LedgerInput,
}

pub struct App {
    pub current_focus: Focus,
    pub previous_focus: Focus,
    pub journal_vec: Vec<journal_struct::JournalData>,
    pub journal_state: ListState,
    pub journal_input: JournalInput,
    pub ledger_input: LedgerInput,
}

impl App {
    pub fn new() -> App {
        let mut buffer = App {
            current_focus: Focus::JournalFrame,
            previous_focus: Focus::JournalFrame,
            journal_vec: Vec::new(),
            journal_state: ListState::default(),
            journal_input: JournalInput::new(),
            ledger_input: LedgerInput::new(),
        };
        buffer.journal_state.select(Some(0));
        buffer
    }

    pub fn get_index_list(&self) -> Vec<String> {
        let mut buffer: Vec<String> = Vec::new();
        for i in 0..self.journal_vec.len() {
            buffer.push(self.journal_vec[i].name.clone())
        }
        buffer
    }

    pub fn new_journal_entry(&mut self, name: String) {
        self.journal_vec.push(JournalData::new(name));
        self.sort_alphabetically();
    }

    pub fn previous(&mut self) {
        if self.journal_vec.len() != 0 {
            let i = match self.journal_state.selected() {
                Some(i) => {
                    if i == 0 {
                        self.journal_state.select(Some(i))
                    } else {
                        self.journal_state.select(Some(i - 1))
                    }
                }
                None => (),
            };
        }
    }

    pub fn next(&mut self) {
        if self.journal_vec.len() != 0 {
            let i = match self.journal_state.selected() {
                Some(i) => {
                    if i == self.get_index_list().len() - 1 {
                        self.journal_state
                            .select(Some(self.get_index_list().len() - 1))
                    } else {
                        self.journal_state.select(Some(i + 1))
                    }
                }
                None => (),
            };
        }
    }

    pub fn export_data_to_json(&self) {
        let mut head = JsonValue::new_array();
        for i in 0..self.journal_vec.len() {
            let journal_buffer = self.journal_vec[i].clone();
            let mut array_buffer = JsonValue::new_array();
            for i in 0..self.journal_vec[i].ledger_vec.len() {
                let mut buffer = JsonValue::new_object();
                buffer["date"] =
                    JsonValue::String(journal_buffer.ledger_vec[i].date.clone()).into();
                buffer["name"] =
                    JsonValue::String(journal_buffer.ledger_vec[i].name.clone()).into();
                buffer["qty"] = JsonValue::String(journal_buffer.ledger_vec[i].qty.clone()).into();
                buffer["price"] =
                    JsonValue::String(journal_buffer.ledger_vec[i].price.clone()).into();
                array_buffer.push(buffer).unwrap();
            }
            let journal_name = journal_buffer.name;
            let mut object_buffer = JsonValue::new_object();
            object_buffer[journal_name] = array_buffer;
            head.push(object_buffer).unwrap();
        }
        let mut file = std::fs::File::create("data.txt").unwrap();
        file.write_all(head.to_string().as_bytes()).unwrap();
    }

    pub fn import_data_from_json(&mut self) {
        let mut file = std::fs::File::open("data.txt").unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        let head_buffer = json::parse(contents.as_str()).unwrap();

        let mut head_array_members = head_buffer.members();
        for journal_id in 0..head_buffer.len() {
            let head_array_buffer = head_array_members.next().unwrap();
            let mut journal_objects = head_array_buffer.entries();
            for _i in 0..journal_objects.len() {
                let (journal_name, ledger_array_iter) = journal_objects.next().unwrap();
                self.new_journal_entry(journal_name.to_string());
                let mut ledger_array_members = ledger_array_iter.members();
                for _i in 0..ledger_array_iter.len() {
                    let ledger_array = ledger_array_members.next().unwrap();
                    let mut ledger_entries = ledger_array.entries();
                    let mut items: Vec<String> = Vec::new();
                    for _i in 0..ledger_entries.len() {
                        let (_unused_var, individual_entry) = ledger_entries.next().unwrap();
                        items.push(individual_entry.to_string());
                    }
                    self.journal_vec[journal_id].new_ledger_entry(
                        items[0].clone(),
                        items[1].clone(),
                        items[2].clone(),
                        items[3].clone(),
                    );
                }
            }
        }
        self.sort_alphabetically();
    }

    pub fn sort_alphabetically(&mut self) {
        let mut continue_loop = true;
        loop {
            if continue_loop == false {
                break;
            }
            continue_loop = bubble_sort_loop(self);
        }
    }
}

pub fn run_app<B: Backend>(terminal: &mut Terminal<B>) -> io::Result<()> {
    let mut app = App::new();

    loop {
        terminal.draw(|f| ui::start_ui(f, &mut app))?;

        if !keycode::read_keys(&mut app) {
            break;
        }
    }
    Ok(())
}

fn bubble_sort_loop(app: &mut App) -> bool {
    let mut continue_loop = false;
    for i in 0..app.journal_vec.len() - 1 {
        let (str_a, str_b, continue_loop_buffer) = compare(
            app.journal_vec[i].name.clone(),
            app.journal_vec[i + 1].name.clone(),
            &mut continue_loop,
        );
        if continue_loop == false {
            continue_loop = continue_loop_buffer;
        }
        app.journal_vec[i].name = str_a;
        app.journal_vec[i + 1].name = str_b;
    }

    continue_loop
}

fn compare(
    str_buffer_a: String,
    str_buffer_b: String,
    continue_loop: &mut bool,
) -> (String, String, bool) {
    let mut str_a = String::new();
    let mut str_b = String::new();
    let mut continue_buffer = true;
    let mut count = 0;
    loop {
        if str_buffer_a.chars().nth(count).unwrap() == str_buffer_b.chars().nth(count).unwrap() {
            if count < str_buffer_a.len() {
                count += 1;
            } else {
                str_a = str_buffer_a;
                str_b = str_buffer_b;
                continue_buffer = false;
                break;
            }
            if count >= str_buffer_b.len() {
                str_a = str_buffer_b;
                str_b = str_buffer_a;
                break;
            }
        } else if str_buffer_a.chars().nth(count).unwrap()
            < str_buffer_b.chars().nth(count).unwrap()
        {
            str_a = str_buffer_a;
            str_b = str_buffer_b.to_string();
            continue_buffer = false;
            break;
        } else {
            str_a = str_buffer_b.to_string();
            str_b = str_buffer_a.to_string();
            break;
        }
    }

    (str_a, str_b, continue_buffer)
}