use chrono::prelude::*;
use home::home_dir;
use serde_json;
use std::collections::HashMap;
use std::fs;
use stybulate::{Table, Style, Cell, Headers};

pub fn get_days_from_month(year: i32, month: u32) -> i64 {
    NaiveDate::from_ymd(
        match month {
            12 => year + 1,
            _ => year,
        },
        match month {
            12 => 1,
            _ => month + 1,
        },
        1,
    )
    .signed_duration_since(NaiveDate::from_ymd(year, month, 1))
    .num_days()
}

fn get_total_months(date: &NaiveDate) -> i32 {
    12 * date.year() + (date.month() as i32)
}

fn add_months(date: NaiveDate, num_months: i32) -> NaiveDate {
    let current_total_months = get_total_months(&date);
    let new_total_months = current_total_months + num_months;
    let years = new_total_months / 12;
    let mut months = new_total_months % 12;
    let mut day = date.day();
    if (months == 2) & (day > 28) {
        months += 1;
        day -= 28;
    }
    if months == 0 {
        NaiveDate::from_ymd(years - 1, 12, day)
    } else {
        NaiveDate::from_ymd(years, months as u32, day)
    }
}

fn relativedelta(start_date: NaiveDate, end_date: NaiveDate) -> (i32, i32, i32) {
    if end_date < start_date {
        panic!(
            "Require end_date ({}) <= start_date ({})",
            end_date, start_date
        )
    }

    let mut months = get_total_months(&end_date) - get_total_months(&start_date);
    let mut years = 0;

    if months.abs() > 11 {
        years = months / 12;
        months = months % 12
    }

    let mut updated_start_date = add_months(start_date, 12 * years + months);
    if updated_start_date > end_date {
        updated_start_date = add_months(updated_start_date, -1);
        months = months - 1;
    }

    let delta_year = end_date.year() - updated_start_date.year();
    let delta_month = end_date.month() - updated_start_date.month();

    let days = if (delta_year == 0) & (delta_month == 0) {
        (end_date.day() - updated_start_date.day()) as i32
    } else if (delta_year == 0) & (delta_month == 1) {
        let days_in_months =
            get_days_from_month(updated_start_date.year(), updated_start_date.month());
        (end_date.day() as i64 + (days_in_months - (updated_start_date.day() as i64))) as i32
    } else if delta_year == 1 {
        (end_date.day() + (31 - updated_start_date.day())) as i32
    } else {
        panic!(
            "Unexpected value for (delta_year, delta_month): ({}, {})",
            delta_year, delta_month
        );
    };

    (years, months, days)
}

fn main() {
    let mut path = home_dir().unwrap();
    let filename = "items.json";
    path.push(filename);
    let items_json_str = fs::read_to_string(path).expect("Unable to read file");
    let items_info: serde_json::Value = serde_json::from_str(&items_json_str).expect("Dodgy JSON");
    let mut durations: HashMap<String, String> = HashMap::new();
    for (item_name, item_dates) in items_info.as_object().unwrap() {
        let now = Utc::now().date().naive_local();
        let purchase_date_str = item_dates.get("bought").unwrap().as_str().unwrap();
        let purchase_date = NaiveDate::parse_from_str(purchase_date_str, "%d/%m/%Y")
            .unwrap_or_else(|error| {
                panic!(
                    "Cannot parse {} using format %d-%m-%Y: {:?}",
                    purchase_date_str, error
                );
            });
        let (years, months, days) = relativedelta(purchase_date, now);
        durations.insert(item_name.to_string(), format!("{} years, {} months, and {} days", years, months, days));
    }
    let mut durations_as_vec = vec![];
    for (key, value) in &durations {
        durations_as_vec.push(vec![key, value])
    }
    durations_as_vec.sort_by(|a, b| b[1].cmp(a[1]));
    let table_cells = durations_as_vec.iter().map(|x| x.iter().map(|y| Cell::from(y)).collect()).collect();

    let table = Table::new(
        Style::FancyPresto,
        table_cells,
        Some(Headers::from(vec!["Item", "Age"])),
    );
    println!("{}", table.tabulate());
}
