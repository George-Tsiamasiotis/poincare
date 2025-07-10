#[derive(thiserror::Error)]
//Custom Error types
pub enum SplineError {
    /// One of the supplied datasets is empty.
    #[error("Supplied `{0}` dataset is empty.")]
    EmptyDataset(Box<str>),
    /// `x` points dataset is not sorted.
    #[error("Supplied x dataset must be sorted.")]
    UnsortedDataset,
    /// 'x' and `y` datasets have differnet length.
    #[error("Supplied datasets must be 1D and of equal length.")]
    DatasetMismatch,
}

impl std::fmt::Debug for SplineError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self)
    }
}
