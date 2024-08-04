#[derive(Debug)]
pub struct LispError {
    message: String,
}

impl LispError {
    pub fn new(message: &str) -> Self {
        LispError {
            message: message.to_string(),
        }
    }
}

impl std::fmt::Display for LispError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for LispError {}
