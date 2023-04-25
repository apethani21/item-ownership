use chrono::prelude::*;
use home::home_dir;
use serde_json;
use std::collections::{HashMap, HashSet};
use std::fs;
use stybulate::{Cell, Headers, Style, Table};

pub fn get_days_from_month(year: i32, month: u32) -> i64 {
    NaiveDate::from_ymd_opt(
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
    .unwrap()
    .signed_duration_since(NaiveDate::from_ymd_opt(year, month, 1).unwrap())
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
    let thirty_day_months: HashSet<i32> = vec![4, 6, 9, 11].into_iter().collect();
    if (months == 2) & (day > 28) {
        months += 1;
        day -= 28;
    }

    if thirty_day_months.contains(&months) && (day == 31) {
        months += 1;
        day = 1;
    }

    if months == 0 {
        NaiveDate::from_ymd_opt(years - 1, 12, day).unwrap()
    } else {
        NaiveDate::from_ymd_opt(years, months as u32, day).unwrap()
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

    if months == -1 {
        years -= 1;
        months = 11;
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

fn parse_date(date: &str) -> NaiveDate {
    NaiveDate::parse_from_str(date, "%d/%m/%Y").unwrap_or_else(|error| {
        panic!("Cannot parse {} using format %d-%m-%Y: {:?}", date, error);
    })
}

fn main() {
    let mut path = home_dir().unwrap();
    let filename = "items.json";
    path.push(filename);
    let items_json_str = fs::read_to_string(path).expect("Unable to read file");
    let items_info: serde_json::Value = serde_json::from_str(&items_json_str).expect("Dodgy JSON");
    let mut durations: HashMap<String, String> = HashMap::new();
    let mut durations_comp: HashMap<String, i32> = HashMap::new();
    for (item_name, item_dates) in items_info.as_object().unwrap() {
        let now = Utc::now().date_naive();
        let purchase_date_str = item_dates.get("bought").unwrap().as_str().unwrap();
        let purchase_date = parse_date(purchase_date_str);
        let stopped_date_str = item_dates.get("stopped").unwrap().as_str();
        let end_date = if let Some(stop_date) = stopped_date_str {
            parse_date(stop_date)
        } else {
            now
        };
        let (years, months, days) = relativedelta(purchase_date, end_date);
        durations.insert(
            item_name.to_string(),
            format!("{} years, {} months, and {} days", years, months, days),
        );
        durations_comp.insert(item_name.to_string(), 365 * years + 31 * months + days);  // rough & ready way to sort by length of ownership
    }
    let mut durations_as_vec = vec![];
    for (key, value) in &durations {
        durations_as_vec.push(vec![key, value])
    }
    durations_as_vec.sort_by(|a, b| {
        durations_comp
            .get(b[0])
            .unwrap()
            .cmp(durations_comp.get(a[0]).unwrap())
    });
    let table_cells = durations_as_vec
        .iter()
        .map(|x| x.iter().map(|y| Cell::from(y)).collect())
        .collect();

    let table = Table::new(
        Style::FancyPresto,
        table_cells,
        Some(Headers::from(vec!["Item", "Age"])),
    );
    println!("{}", table.tabulate());
}
