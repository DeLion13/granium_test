use thiserror::Error;

/// Alias for a standard Result, but with [`GraniumError`]
pub type GraniumResult<T> = Result<T, GraniumError>;

/// Highlevel error, that helps to cover all results
/// and avoid unwrapping and panicks everywhere
#[derive(Error, Debug)]
pub enum GraniumError {
    #[error("Failed to read env var")]
    ReadEnvVariable {
        #[source]
        source: std::env::VarError,
    },

    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),
}
