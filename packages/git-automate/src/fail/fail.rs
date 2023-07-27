#[derive(Debug, Clone, Copy)]
pub struct IOFail {
    pub message: &'static str,
}

impl From<std::io::Error> for IOFail {
    fn from(value: std::io::Error) -> Self {
        Self { message: format_args!("\nStderr returned: {value}\n").as_str().unwrap() }
    }
}

impl std::fmt::Display for IOFail {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self.message)
    }
}