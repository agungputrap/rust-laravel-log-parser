use crate::models::{AppError, LogEntry};

pub fn parse_line(line: &str, id: u64) -> Option<LogEntry> {
    let ts_start = line.find('[')? + 1;
    let ts_end = line.find(']')?;
    let timestamp = line[ts_start..ts_end].to_string();

    let remaining = &line[ts_end + 1..];
    let level_end = remaining.find(':')?;
    let level_part = remaining[..level_end].trim();

    let level = level_part.split('.').next()?.to_string();

    let message_part = remaining[level_end + 1..].split('{').next()?.trim();

    let error_type = if message_part.contains("SQLSTATE") || message_part.contains("Database") {
        AppError::DatabaseError(message_part.to_string())
    } else if message_part.contains("Connection refused") || message_part.contains("cURL error") {
        AppError::ConnectionRefused(message_part.to_string())
    } else if message_part.contains("Unable to locate Mix file") {
        AppError::ViewError(message_part.to_string())
    } else {
        AppError::GeneralError(message_part.to_string())
    };

    Some(LogEntry {
        id,
        timestamp,
        level,
        error_type,
    })
}
