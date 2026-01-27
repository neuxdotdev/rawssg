use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;
use chrono::{DateTime, Utc};
use crate::config::RawConfig;
use crate::error::{RawSSGError, Result};
use crate::frontmatter::Content;
pub struct Builder {
    config: RawConfig,
    template_engine: crate::template::TemplateEngine,
    build_time: DateTime<Utc>,
}
impl Builder {
    pub fn new(config: RawConfig) -> Result<Self> {
        let template_engine = crate::template::TemplateEngine::new(&config)?;
        let build_time = Utc::now();
        Ok(Self {
                config,
                template_engine,
                build_time,
            })
        }
        pub fn build(&self) -> Result<()> {
            let output_dir = Path::new(&self.config.build.output_dir);
            if self.config.build.clean_build && output_dir.exists() {
                fs::remove_dir_all(output_dir)?;
            }
            fs::create_dir_all(output_dir)?;
            let content_files = self.find_content_files()?;
            if content_files.is_empty() {
                return Err(RawSSGError::Build("No content files found".to_string()));
            }
            let mut pages = Vec::new();
            for file in content_files {
                let content = Content::parse(&file)?;
                if content.should_skip(self.config.content.drafts) {
                    continue;
                }
                pages.push(content);
            }
            pages.sort_by(|a, b| {
                    let empty = String::new();
                    let a_date = a.frontmatter.date.as_ref().unwrap_or(&empty);
                    let b_date = b.frontmatter.date.as_ref().unwrap_or(&empty);
                    b_date.cmp(a_date)
                });
                let context = self.build_context(&pages)?;
                for page in &pages {
                    self.render_page(page, &context, output_dir)?;
                }
                self.copy_static_assets(output_dir)?;
                if self.config.build.generate.sitemap {
                    self.generate_sitemap(&pages, output_dir)?;
                }
                if self.config.build.generate.robots {
                    self.generate_robots_txt(output_dir)?;
                }
                if self.config.build.generate.feed {
                    self.generate_feed(&pages, output_dir)?;
                }
                self.write_build_info(output_dir)?;
                Ok(())
            }
            fn find_content_files(&self) -> Result<Vec<PathBuf>> {
                let mut files = Vec::new();
                let default_extensions = vec!["rw".to_string(), "md".to_string()];
                let default_ignore = vec!["drafts/".to_string()];
                let extensions = self.config.content.extensions
                .as_ref()
                .unwrap_or(&default_extensions);
                let ignore_patterns = self.config.content.ignore
                .as_ref()
                .unwrap_or(&default_ignore);
                for entry in WalkDir::new(".")
                .follow_links(true)
                .into_iter()
                .filter_map(|e| e.ok())
                {
                    let path = entry.path();
                    if path.is_file() {
                        if let Some(ext) = path.extension() {
                            let ext_str = ext.to_string_lossy();
                            if extensions.iter().any(|e| e == &ext_str) {
                                let path_str = path.to_string_lossy();
                                if !ignore_patterns.iter().any(|p| path_str.contains(p)) {
                                    files.push(path.to_path_buf());
                                }
                            }
                        }
                    }
                }
                Ok(files)
            }
            fn build_context(&self, pages: &[Content]) -> Result<HashMap<String, serde_json::Value>> {
                let mut context = HashMap::new();
                let config_json = serde_json::to_value(&self.config)?;
                context.insert("config".to_string(), config_json);
                context.insert("build_time".to_string(),
                    serde_json::Value::String(self.build_time.to_rfc3339()));
                let pages_json: Vec<serde_json::Value> = pages
                .iter()
                .filter_map(|page| {
                        if page.should_skip(self.config.content.drafts) {
                            return None;
                        }
                        let mut page_obj = HashMap::new();
                        page_obj.insert("title".to_string(),
                            serde_json::Value::String(
                                page.frontmatter.title.clone()
                                .unwrap_or_else(|| page.path.file_stem()
                                    .unwrap()
                                    .to_string_lossy()
                                    .to_string())
                            ));
                            if let Some(slug) = &page.frontmatter.slug {
                                page_obj.insert("slug".to_string(),
                                    serde_json::Value::String(slug.clone()));
                            }
                            if let Some(date) = &page.frontmatter.date {
                                page_obj.insert("date".to_string(),
                                    serde_json::Value::String(date.clone()));
                            }
                            serde_json::to_value(page_obj).ok()
                        })
                        .collect();
                        context.insert("pages".to_string(), serde_json::Value::Array(pages_json));
                        Ok(context)
                    }
                    fn render_page(
                        &self,
                        page: &Content,
                        context: &HashMap<String, serde_json::Value>,
                        output_dir: &Path,
                    ) -> Result<()> {
                    let output_path = page.output_path(output_dir);
                    if let Some(parent) = output_path.parent() {
                        fs::create_dir_all(parent)?;
                    }
                    let html = self.template_engine.render_content(page, context)?;
                    let html = self.template_engine.minify_html(&html);
                    fs::write(&output_path, html)?;
                    println!("Generated: {}", output_path.display());
                    Ok(())
                }
                fn copy_static_assets(&self, output_dir: &Path) -> Result<()> {
                    let static_dir = Path::new(&self.config.template.dir).join("static");
                    if static_dir.exists() {
                        for entry in WalkDir::new(&static_dir) {
                            let entry = entry?;
                            let src_path = entry.path();
                            let rel_path = src_path.strip_prefix(&static_dir)?;
                            let dst_path = output_dir.join(rel_path);
                            if src_path.is_dir() {
                                fs::create_dir_all(&dst_path)?;
                            } else {
                            fs::copy(src_path, dst_path)?;
                        }
                    }
                }
                Ok(())
            }
            fn generate_sitemap(&self, pages: &[Content], output_dir: &Path) -> Result<()> {
                let base_url = self.config.base_url
                .as_ref()
                .ok_or_else(|| RawSSGError::Build("Base URL required for sitemap".to_string()))?;
                let mut sitemap = String::new();
                sitemap.push_str("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n");
                sitemap.push_str("<urlset xmlns=\"http://www.sitemaps.org/schemas/sitemap/0.9\">\n");
                for page in pages {
                    if page.should_skip(self.config.content.drafts) {
                        continue;
                    }
                    let slug = page.frontmatter.slug.clone().unwrap_or_else(|| {
                            page.path.file_stem()
                            .unwrap()
                            .to_string_lossy()
                            .to_string()
                        });
                        let url = if slug == "index" {
                            format!("{}/", base_url)
                        } else {
                        format!("{}/{}/", base_url, slug)
                    };
                    sitemap.push_str(&format!("  <url>\n    <loc>{}</loc>\n", url));
                    if let Some(date) = &page.frontmatter.date {
                        if let Ok(parsed) = chrono::NaiveDate::parse_from_str(date, "%Y-%m-%d") {
                            let lastmod = parsed.and_hms_opt(0, 0, 0).unwrap();
                            sitemap.push_str(&format!("    <lastmod>{}</lastmod>\n",
                                    lastmod.format("%Y-%m-%d")));
                        }
                    }
                    sitemap.push_str("  </url>\n");
                }
                sitemap.push_str("</urlset>");
                let sitemap_path = output_dir.join("sitemap.xml");
                fs::write(sitemap_path, sitemap)?;
                println!("Generated: sitemap.xml");
                Ok(())
            }
            fn generate_robots_txt(&self, output_dir: &Path) -> Result<()> {
                let base_url = self.config.base_url
                .as_ref()
                .ok_or_else(|| RawSSGError::Build("Base URL required for robots.txt".to_string()))?;
                let robots = format!(
                    "User-agent: *\n\
                    Allow: /\n\
                    \n\
                    Sitemap: {}/sitemap.xml",
                    base_url.trim_end_matches('/')
                );
                let robots_path = output_dir.join("robots.txt");
                fs::write(robots_path, robots)?;
                println!("Generated: robots.txt");
                Ok(())
            }
            fn generate_feed(&self, pages: &[Content], output_dir: &Path) -> Result<()> {
                let base_url = self.config.base_url
                .as_ref()
                .ok_or_else(|| RawSSGError::Build("Base URL required for feed".to_string()))?;
                let mut feed = String::new();
                feed.push_str("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n");
                feed.push_str("<rss version=\"2.0\">\n");
                feed.push_str("  <channel>\n");
                feed.push_str(&format!("    <title>{}</title>\n", self.config.title));
                if let Some(desc) = &self.config.description {
                    feed.push_str(&format!("    <description>{}</description>\n", desc));
                }
                feed.push_str(&format!("    <link>{}</link>\n", base_url));
                feed.push_str(&format!("    <lastBuildDate>{}</lastBuildDate>\n",
                        self.build_time.to_rfc2822()));
                for page in pages.iter().take(10) {
                    if page.should_skip(self.config.content.drafts) {
                        continue;
                    }
                    let title = page.frontmatter.title.clone().unwrap_or_else(|| {
                            page.path.file_stem()
                            .unwrap()
                            .to_string_lossy()
                            .to_string()
                        });
                        let slug = page.frontmatter.slug.clone().unwrap_or_else(|| {
                                page.path.file_stem()
                                .unwrap()
                                .to_string_lossy()
                                .to_string()
                            });
                            let url = if slug == "index" {
                                base_url.clone()
                            } else {
                            format!("{}/{}/", base_url, slug)
                        };
                        feed.push_str("    <item>\n");
                        feed.push_str(&format!("      <title>{}</title>\n", title));
                        feed.push_str(&format!("      <link>{}</link>\n", url));
                        if let Some(date) = &page.frontmatter.date {
                            if let Ok(parsed) = chrono::NaiveDate::parse_from_str(date, "%Y-%m-%d") {
                                let pub_date = parsed.and_hms_opt(0, 0, 0).unwrap();
                                let pub_date_dt = chrono::DateTime::<chrono::Utc>::from_naive_utc_and_offset(pub_date, chrono::Utc);
                                feed.push_str(&format!("      <pubDate>{}</pubDate>\n",
                                        pub_date_dt.to_rfc2822()));
                            }
                        }
                        let description = page.body.chars().take(200).collect::<String>();
                        feed.push_str(&format!("      <description>{}</description>\n",
                                html_escape::encode_text(&description)));
                        feed.push_str("    </item>\n");
                    }
                    feed.push_str("  </channel>\n");
                    feed.push_str("</rss>");
                    let feed_path = output_dir.join("feed.xml");
                    fs::write(feed_path, feed)?;
                    println!("Generated: feed.xml");
                    Ok(())
                }
                fn write_build_info(&self, output_dir: &Path) -> Result<()> {
                    let build_info = serde_json::json!({
                            "generator": "RawSSG",
                            "version": env!("CARGO_PKG_VERSION"),
                            "build_time": self.build_time.to_rfc3339(),
                            "config_version": self.config.version,
                            "pages_count": 0,
                        });
                        let build_info_str = serde_json::to_string_pretty(&build_info)?;
                        let build_info_path = output_dir.join("build-info.json");
                        fs::write(build_info_path, build_info_str)?;
                        println!("Build completed successfully at {}", self.build_time);
                        Ok(())
                    }
                }