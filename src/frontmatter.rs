use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::error::{RawSSGError, Result};
const FRONTMATTER_DELIMITER: &str = "---";
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Frontmatter {
    pub title: Option<String>,
    pub slug: Option<String>,
    pub layout: Option<String>,
    pub template: Option<String>,
    pub date: Option<String>,
    pub draft: Option<bool>,
    pub tags: Option<Vec<String>>,
    pub categories: Option<Vec<String>>,
    #[serde(flatten)]
    pub custom: HashMap<String, serde_yaml::Value>,
}
#[derive(Debug, Clone)]
pub struct Content {
    pub frontmatter: Frontmatter,
    pub body: String,
    pub path: std::path::PathBuf,
}
impl Content {
    pub fn parse<P: AsRef<std::path::Path>>(path: P) -> Result<Self> {
        let path = path.as_ref();
        let raw = std::fs::read_to_string(path)?;
        let (frontmatter, body) = if raw.starts_with(FRONTMATTER_DELIMITER) {
            let parts: Vec<&str> = raw.splitn(3, FRONTMATTER_DELIMITER).collect();
            if parts.len() < 3 {
                return Err(RawSSGError::Frontmatter(
                        "Invalid frontmatter format".to_string(),
                    ));
                }
                let fm_str = parts[1].trim();
                let body = parts[2].trim_start().to_string();
                let frontmatter: Frontmatter = serde_yaml::from_str(fm_str)?;
                (frontmatter, body)
            } else {
            (Frontmatter::default(), raw.clone())
        };
        Ok(Self {
                frontmatter,
                body,
                path: path.to_path_buf(),
            })
        }
        pub fn should_skip(&self, show_drafts: bool) -> bool {
            if let Some(draft) = self.frontmatter.draft {
                draft && !show_drafts
            } else {
            false
        }
    }
    pub fn output_path(&self, base: &std::path::Path) -> std::path::PathBuf {
        let slug = self.frontmatter.slug.clone().unwrap_or_else(|| {
                self.path.file_stem()
                .unwrap()
                .to_string_lossy()
                .to_string()
            });
            if slug == "index" {
                base.join("index.html")
            } else {
            base.join(&slug).join("index.html")
        }
    }
}