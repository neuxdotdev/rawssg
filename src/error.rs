use thiserror::Error;
use handlebars::{RenderError, TemplateError};
#[derive(Error, Debug)]
pub enum RawSSGError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("JSON parsing error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("YAML parsing error: {0}")]
    Yaml(#[from] serde_yaml::Error),
    #[error("Template error: {0}")]
    Template(#[from] TemplateError),
    #[error("Template render error: {0}")]
    Render(#[from] RenderError),
    #[error("Config error: {0}")]
    Config(String),
    #[error("Frontmatter error: {0}")]
    Frontmatter(String),
    #[error("Build error: {0}")]
    Build(String),
    #[error("Watcher error: {0}")]
    Watcher(#[from] notify::Error),
    #[error("Walkdir error: {0}")]
    Walkdir(#[from] walkdir::Error),
    #[error("Path strip prefix error: {0}")]
    StripPrefix(#[from] std::path::StripPrefixError),
}
pub type Result<T> = std::result::Result<T, RawSSGError>;