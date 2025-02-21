use std::{fmt::Display, sync::Arc};

pub type Result<T, E = GuiError> = std::result::Result<T, E>;

#[derive(Debug, Clone)]
pub struct GuiError(Arc<str>);

impl Display for GuiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl<E> From<E> for GuiError
where
    E: std::error::Error,
{
    fn from(error: E) -> Self {
        Self::new(error.to_string())
    }
}

impl GuiError {
    pub fn new(message: String) -> Self {
        return Self(Arc::from(message));
    }
}
