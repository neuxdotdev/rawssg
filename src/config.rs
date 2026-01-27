use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;
use crate::error::{RawSSGError, Result};
use chrono::Datelike;
const CONFIG_VERSION: &str = "0.0.2";
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct RawConfig {
    pub version: String,
    pub title: String,
    pub description: Option<String>,
    pub favicon: Option<String>,
    pub base_url: Option<String>,
    #[serde(default)]
    pub content: ContentConfig,
    #[serde(default)]
    pub build: BuildConfig,
    #[serde(default)]
    pub template: TemplateConfig,
    #[serde(default)]
    pub nav: Vec<NavItem>,
    #[serde(default)]
    pub footer: FooterConfig,
    #[serde(default)]
    pub custom: HashMap<String, serde_json::Value>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "kebab-case")]
pub struct ContentConfig {
    pub format: Option<String>,
    pub extensions: Option<Vec<String>>,
    pub ignore: Option<Vec<String>>,
    pub drafts: bool,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct BuildConfig {
    #[serde(default = "default_output_dir")]
    pub output_dir: String,
    #[serde(default)]
    pub clean_build: bool,
    #[serde(default)]
    pub minify: MinifyConfig,
    #[serde(default)]
    pub generate: GenerateConfig,
    #[serde(default)]
    pub strict: bool,
}
impl Default for BuildConfig {
    fn default() -> Self {
        Self {
            output_dir: default_output_dir(),
            clean_build: false,
            minify: MinifyConfig::default(),
            generate: GenerateConfig::default(),
            strict: false,
        }
    }
}
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MinifyConfig {
    pub html: bool,
    pub css: bool,
    pub js: bool,
}
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GenerateConfig {
    pub sitemap: bool,
    pub robots: bool,
    pub feed: bool,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateConfig {
    #[serde(default = "default_template_dir")]
    pub dir: String,
    #[serde(default = "default_layout")]
    pub default_layout: String,
    #[serde(default)]
    pub partials: Vec<String>,
}
impl Default for TemplateConfig {
    fn default() -> Self {
        Self {
            dir: default_template_dir(),
            default_layout: default_layout(),
            partials: Vec::new(),
        }
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NavItem {
    pub title: String,
    pub url: String,
    #[serde(default)]
    pub items: Vec<NavItem>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FooterConfig {
    pub text: Option<String>,
    #[serde(default)]
    pub links: Vec<NavItem>,
    pub copyright: Option<String>,
}
fn default_output_dir() -> String {
    "dist".to_string()
}
fn default_template_dir() -> String {
    "templates".to_string()
}
fn default_layout() -> String {
    "base.html".to_string()
}
impl RawConfig {
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self> {
        let content = std::fs::read_to_string(path)?;
        let config: Self = serde_json::from_str(&content)
        .or_else(|_| serde_yaml::from_str(&content))?;
        config.validate()?;
        Ok(config)
    }
    pub fn validate(&self) -> Result<()> {
        if self.version != CONFIG_VERSION {
            return Err(RawSSGError::Config(format!(
                        "Config version {} does not match expected version {}",
                        self.version, CONFIG_VERSION
                    )));
                }
                if self.build.strict {
                    if self.title.is_empty() {
                        return Err(RawSSGError::Config("Title is required in strict mode".into()));
                    }
                    for (i, item) in self.nav.iter().enumerate() {
                        if item.url.is_empty() {
                            return Err(RawSSGError::Config(format!(
                                        "Navigation item {} has empty URL", i
                                    )));
                                }
                            }
                        }
                        Ok(())
                    }
                    pub fn save<P: AsRef<Path>>(&self, path: P) -> Result<()> {
                        let json = serde_json::to_string_pretty(self)?;
                        std::fs::write(path, json)?;
                        Ok(())
                    }
                    pub fn default_config() -> Self {
                        Self {
                            version: CONFIG_VERSION.to_string(),
                            title: "RawSSG Site".to_string(),
                            description: Some("A site generated with RawSSG".to_string()),
                            favicon: Some("favicon.ico".to_string()),
                            base_url: Some("http://localhost:8080".to_string()),
                            content: ContentConfig {
                                format: Some("plain".to_string()),
                                extensions: Some(vec!["rw".to_string(), "md".to_string()]),
                                ignore: Some(vec!["drafts/*".to_string()]),
                                drafts: false,
                            },
                            build: BuildConfig::default(),
                            template: TemplateConfig::default(),
                            nav: vec![
                                NavItem {
                                    title: "Home".to_string(),
                                    url: "/".to_string(),
                                    items: Vec::new(),
                                },
                                NavItem {
                                    title: "About".to_string(),
                                    url: "/about".to_string(),
                                    items: Vec::new(),
                                },
                            ],
                            footer: FooterConfig {
                                text: Some("Built with RawSSG".to_string()),
                                links: vec![
                                    NavItem {
                                        title: "GitHub".to_string(),
                                        url: "https://github.com/neuxdotdev".to_string(),
                                        items: Vec::new(),
                                    },
                                ],
                                copyright: Some(format!("© {} Your Name", chrono::Local::now().year())),
                            },
                            custom: HashMap::new(),
                        }
                    }
                }