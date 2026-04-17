/// Represent type of errors
#[derive(Debug, PartialEq)]
pub enum AppError {
    DatabaseError(String),
    ConnectionRefused(String),
    ViewError(String),
    GeneralError(String),
}

#[derive(Debug, PartialEq)]
pub struct LogEntry {
    pub id: u64,
    pub timestamp: String,
    pub level: String,
    pub error_type: AppError,
}
