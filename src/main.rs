mod models;
mod parser;

use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let args: Vec<String> = env::args().collect();

    let filter_level = if args.len() > 1 {
        Some(args[1].to_string())
    } else {
        None
    };

    if let Some(ref level) = filter_level {
        println!("Finding log with level: {}", level);
    }

    let path = "laravel.log";
    let file = File::open(path).expect("Failed to open log file!");
    let reader = BufReader::new(file);

    let mut current_log_buffer = String::new();
    let mut log_count = 0;

    for line in reader.lines() {
        if let Ok(line_content) = line {
            if is_real_header(&line_content) {
                if !current_log_buffer.is_empty() {
                    process_and_filter(&current_log_buffer, log_count, &filter_level);
                    log_count += 1;
                }

                current_log_buffer = line_content;
            } else {
                current_log_buffer.push_str("\n");
                current_log_buffer.push_str(&line_content);
            }
        }
    }

    if !current_log_buffer.is_empty() {
        process_and_filter(&current_log_buffer, log_count, &filter_level);
    }
}

fn is_real_header(line: &str) -> bool {
    if line.len() < 21 {
        return false;
    }

    let chars: Vec<char> = line.chars().take(11).collect();
    line.starts_with('[') && chars[1].is_ascii_digit() && chars[5] == '-' && chars[8] == '-'
}

fn process_and_filter(full_log: &str, id: u64, filter: &Option<String>) {
    if let Some(entry) = parser::parse_line(full_log, id) {
        match filter {
            Some(target_level) => {
                if &entry.level == target_level {
                    print_log_summary(entry);
                }
            }
            None => {
                print_log_summary(entry);
            }
        }
    }
}

fn print_log_summary(entry: models::LogEntry) {
    use models::AppError;

    match entry.error_type {
        AppError::DatabaseError(msg) => {
            println!("[{}] DB_ERROR: {}", entry.timestamp, msg);
        }
        AppError::ConnectionRefused(msg) => {
            println!("[{}] CONNECTION_REFUSED: {}", entry.timestamp, msg);
        }
        AppError::ViewError(msg) => {
            println!("[{}] VIEW_ERROR: {}", entry.timestamp, msg);
        }
        AppError::GeneralError(msg) => {
            println!("[{}] GENERAL_ERROR: {}", entry.timestamp, msg);
        }
    }
}
