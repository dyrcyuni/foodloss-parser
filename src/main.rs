#![allow(unused_imports)]
use std::{collections::HashMap, fs, io::BufRead};

fn main() {
    let file = fs::OpenOptions::new()
        .read(true)
        .open("../FoodLoss.csv")
        .expect("Failed to open file");

    let mut invalid_count = 0;

    let mut values = HashMap::<String, u32>::new();

    for (i, line) in std::io::BufReader::new(file).lines().skip(1).enumerate() {
        let line = line.expect("Failed to read line");
        let line_no = i + 2; // 1-indexed + heading

        let Some(record) = parse_record(&line) else {
            eprintln!("{line_no}: {}", line);
            invalid_count += 1;
            continue;
        };

        // do something with record here
        // println!("{:#?}", record);
        // println!("{:4} {}", record.cpc_code, record.commodity);

        let value = format!("{} {}", record.cpc_code, record.commodity);
        let entry = values.entry(value.to_string()).or_insert(0);
        *entry += 1;
    }

    // println!("Total invalid records found: {invalid_count}");

    // println!("----------");
    for (entry, count) in &values {
        // println!("{count:4}: {entry}");
        println!("{entry}");
    }
    // println!("Unique: {}", values.len());
    // println!("Total: {}", values.values().sum::<u32>());
}

#[allow(dead_code)]
#[derive(Debug)]
struct Record {
    pub m49code: String,
    pub country: String,
    pub region: Option<String>,
    pub cpc_code: String,
    pub commodity: String,
    pub year: u32,
    pub loss_percentage: f32,
    pub activity: Option<String>,
    pub food_supply_stage: Option<String>,
    pub cause_of_loss: Option<String>,
}

fn parse_record(line: &str) -> Option<Record> {
    let mut rows = split_rows(&line).into_iter();

    macro_rules! next {
        (String) => {{
            rows.next().filter(|cell| !cell.is_empty())?.to_string()
        }};
        (Option<String>) => {{
            rows.next()
                .filter(|cell| !cell.is_empty())
                .map(|cell| cell.to_string())
        }};
        (u32) => {{
            rows.next()?.parse::<u32>().ok()?
        }};
        (f32) => {{
            rows.next()?.parse::<f32>().ok()?
        }};
    }

    let record = Record {
        m49code: next!(String),
        country: next!(String),
        region: next!(Option<String>),
        cpc_code: next!(String),
        commodity: next!(String),
        year: next!(u32),
        loss_percentage: next!(f32),
        activity: next!(Option<String>),
        food_supply_stage: next!(Option<String>),
        cause_of_loss: next!(Option<String>),
    };

    // 3 rows of irrelevant data
    rows.nth(2);
    // check for no extra cells
    if rows.next().is_some() {
        return None;
    }

    Some(record)
}

fn split_rows(line: &str) -> Vec<&str> {
    let mut rows = Vec::new();

    let mut start = 0;
    let mut end = 0;
    let mut is_quote = false;

    let mut chars = line.chars();
    while let Some(ch) = chars.next() {
        match ch {
            ',' if !is_quote => {
                rows.push(line.get(start..end).unwrap_or(""));
                start = end + 1;
                end = start;
            }
            '"' => {
                if !is_quote {
                    start += 1;
                    is_quote = true;
                } else {
                    match chars.next() {
                        // escaped quote with `""`
                        Some('"') => {
                            end += 1;
                        }
                        // eol or comma
                        None | Some(',') => {
                            end += 1;
                            rows.push(line.get(start..end).unwrap_or(""));
                            start = end + 2;
                            end = start;
                            is_quote = false;
                        }
                        Some(next_ch) => {
                            panic!("unexpected character `{}` after closing quote", next_ch)
                        }
                    }
                }
            }
            _ => {
                end += 1;
            }
        }
    }

    if is_quote {
        panic!("unexpected end of line, expected closing quote (multi-line values not supported)");
    }
    rows.push(line.get(start..end).unwrap_or(""));
    rows
}
