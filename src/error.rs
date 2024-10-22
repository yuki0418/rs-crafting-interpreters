#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("an internal server error occurred")]
    Anyhow(#[from] anyhow::Error),

    #[error("[line {0}] Error {0}: {0}")]
    ScannerError(usize, usize, String),
}
