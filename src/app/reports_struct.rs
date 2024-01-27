use chrono::{NaiveDate, Datelike};

use super::App;

struct Report {
    weekly: Vec<Vec<i32>>,
}

pub fn generate_weekly_report(week: u32, app: &App) {

    let mut report_buffer = Report {weekly: Vec::new()};
    for i in 0..app.journal_vec.len() {
        let mut ledger_report_buffer: Vec<i32> = Vec::new();
        let mut working_date = chrono::offset::Local::now().naive_local();
        let current_date = chrono::offset::Local::now().naive_local().iso_week();

        for j in 0..app.journal_vec[i].ledger_vec.len() {
            let date = NaiveDate::parse_from_str(&app.journal_vec[i].ledger_vec[j].date, "%Y-%m-%d").unwrap();
            if date < working_date.date() {
                
            }

        }
    }
}