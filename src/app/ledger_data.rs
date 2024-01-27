use chrono::{Local, DateTime};


#[derive(Clone)]
pub struct LedgerData {
    pub date: String,
    pub name: String,
    pub qty: String,
    pub price: String,
}

impl LedgerData {
    pub fn new(date: String, name: String, qty: String, price: String) -> LedgerData {
        LedgerData { date: date, name: name, qty: qty, price: price }
    }
}

pub enum LedgerInputSelection {
    Date,
    Name,
    Qty,
    Price,
}
pub struct LedgerInput {
    pub date: String,
    pub name: String,
    pub qty: String,
    pub price: String,
    pub input_selection: LedgerInputSelection,
}

impl LedgerInput {
    pub fn new() -> LedgerInput {
        LedgerInput {
            date: get_current_date(),
            name: String::new(),
            qty: String::new(),
            price: String::new(),
            input_selection: LedgerInputSelection::Name,
        }
    }

    pub fn retrieve_input_items(&self) -> (&String, &String, &String, &String) {
        (&self.date, &self.name, &self.qty, &self.price)
    }

    pub fn clear(&mut self) {
        self.date = get_current_date();
        self.name.clear();
        self.qty.clear();
        self.price.clear();
    }
}

fn get_current_date() -> String {
    let buffer: DateTime<Local> = Local::now();
    let buffer = buffer.naive_local().date();
    buffer.to_string()
    
}