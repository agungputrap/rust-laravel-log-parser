/// Represent type of errors
pub enum AppError {
    DatabaseError(String),
    ConnectionRefused(String),
    ViewError(String),
    GeneralError(String),
}

pub struct LogEntry {
    pub id: u64,
    pub timestamp: String,
    pub level: String,
    pub error_type: AppError,
}
