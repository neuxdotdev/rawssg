use handlebars::{Context, Handlebars, Helper, HelperDef, Output, RenderContext, RenderError};
use include_dir::{include_dir, Dir};
use serde::Serialize;
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use crate::config::RawConfig;
use crate::error::Result;
use crate::frontmatter::Content;
use crate::error::RawSSGError;

// Include default templates from the correct location
// Pastikan folder templates ada di src/templates
static TEMPLATE_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/src/templates");

pub struct TemplateEngine {
    handlebars: Handlebars<'static>,
    config: RawConfig,
}

impl TemplateEngine {
    pub fn new(config: &RawConfig) -> Result<Self> {
        let mut handlebars = Handlebars::new();
        Self::register_helpers(&mut handlebars);
        let template_dir = Path::new(&config.template.dir);
        
        // Always ensure template directory exists and has required templates
        Self::ensure_templates(template_dir)?;
        Self::register_templates(&mut handlebars, template_dir)?;
        
        Ok(Self {
            handlebars,
            config: config.clone(),
        })
    }
    
    fn ensure_templates(dir: &Path) -> Result<()> {
        // Create directory if it doesn't exist
        fs::create_dir_all(dir)?;
        
        // Check if essential templates exist
        let essential_templates = vec![
            "base.html",
            "_sidebar.html",
            "_content.html",
            "_footer.html",
        ];
        
        let missing_essentials = essential_templates.iter()
            .any(|&template| !dir.join(template).exists());
        
        // If any essential template is missing, extract all default templates
        if missing_essentials {
            Self::extract_default_templates(dir)?;
        }
        
        // Ensure static directories exist
        let static_dirs = vec!["static/css", "static/js"];
        for subdir in static_dirs {
            let dir_path = dir.join(subdir);
            if !dir_path.exists() {
                fs::create_dir_all(&dir_path)?;
            }
        }
        
        Ok(())
    }
    
    fn extract_default_templates(dir: &Path) -> Result<()> {
        // Extract all files from the included template directory
        Self::extract_dir_contents(&TEMPLATE_DIR, dir)?;
        Ok(())
    }
    
    fn extract_dir_contents(source_dir: &Dir, dest_dir: &Path) -> Result<()> {
        for entry in source_dir.entries() {
            match entry {
                include_dir::DirEntry::Dir(subdir) => {
                    let dir_name = subdir.path()
                        .file_name()
                        .map(|s| s.to_string_lossy().to_string())
                        .unwrap_or_else(|| "".to_string());
                    
                    if dir_name.is_empty() {
                        continue;
                    }
                    
                    let dest_subdir = dest_dir.join(&dir_name);
                    fs::create_dir_all(&dest_subdir)?;
                    Self::extract_dir_contents(subdir, &dest_subdir)?;
                }
                include_dir::DirEntry::File(file) => {
                    let file_name = file.path()
                        .file_name()
                        .map(|s| s.to_string_lossy().to_string())
                        .unwrap_or_else(|| "".to_string());
                    
                    if file_name.is_empty() {
                        continue;
                    }
                    
                    let dest_path = dest_dir.join(&file_name);
                    
                    // Only write if file doesn't exist or is different
                    let should_write = if dest_path.exists() {
                        let existing_content = fs::read_to_string(&dest_path)
                            .unwrap_or_default();
                        let new_content = file.contents_utf8().unwrap_or("");
                        existing_content != new_content
                    } else {
                        true
                    };
                    
                    if should_write {
                        if let Some(content) = file.contents_utf8() {
                            fs::write(&dest_path, content)?;
                            println!("Created template: {}", dest_path.display());
                        }
                    }
                }
            }
        }
        Ok(())
    }
    
    fn register_helpers(handlebars: &mut Handlebars<'static>) {
        // Helper for equality check
        struct EqHelper;
        
        impl HelperDef for EqHelper {
            fn call<'reg: 'rc, 'rc>(
                &self,
                h: &Helper<'rc>,
                _: &'reg Handlebars<'reg>,
                _: &'rc Context,
                _: &mut RenderContext<'reg, 'rc>,
                out: &mut dyn Output,
            ) -> std::result::Result<(), RenderError> {
                // Menggunakan owned String untuk menghindari temporary references
                let a_str = h.param(0)
                    .and_then(|v| v.value().as_str())
                    .map(|s| s.to_string());
                let a_int = h.param(0)
                    .and_then(|v| v.value().as_i64())
                    .map(|n| n.to_string());
                
                let a = a_str.or(a_int).unwrap_or_default();
                
                let b_str = h.param(1)
                    .and_then(|v| v.value().as_str())
                    .map(|s| s.to_string());
                let b_int = h.param(1)
                    .and_then(|v| v.value().as_i64())
                    .map(|n| n.to_string());
                
                let b = b_str.or(b_int).unwrap_or_default();
                
                let result = a == b;
                out.write(&result.to_string())?;
                Ok(())
            }
        }
        
        // Helper for not equal
        struct NeqHelper;
        
        impl HelperDef for NeqHelper {
            fn call<'reg: 'rc, 'rc>(
                &self,
                h: &Helper<'rc>,
                _: &'reg Handlebars<'reg>,
                _: &'rc Context,
                _: &mut RenderContext<'reg, 'rc>,
                out: &mut dyn Output,
            ) -> std::result::Result<(), RenderError> {
                // Menggunakan owned String untuk menghindari temporary references
                let a_str = h.param(0)
                    .and_then(|v| v.value().as_str())
                    .map(|s| s.to_string());
                let a_int = h.param(0)
                    .and_then(|v| v.value().as_i64())
                    .map(|n| n.to_string());
                
                let a = a_str.or(a_int).unwrap_or_default();
                
                let b_str = h.param(1)
                    .and_then(|v| v.value().as_str())
                    .map(|s| s.to_string());
                let b_int = h.param(1)
                    .and_then(|v| v.value().as_i64())
                    .map(|n| n.to_string());
                
                let b = b_str.or(b_int).unwrap_or_default();
                
                let result = a != b;
                out.write(&result.to_string())?;
                Ok(())
            }
        }
        
        // Helper for lowercase
        struct LowercaseHelper;
        
        impl HelperDef for LowercaseHelper {
            fn call<'reg: 'rc, 'rc>(
                &self,
                h: &Helper<'rc>,
                _: &'reg Handlebars<'reg>,
                _: &'rc Context,
                _: &mut RenderContext<'reg, 'rc>,
                out: &mut dyn Output,
            ) -> std::result::Result<(), RenderError> {
                if let Some(s) = h.param(0).and_then(|v| v.value().as_str()) {
                    out.write(&s.to_lowercase())?;
                }
                Ok(())
            }
        }
        
        // Helper for date formatting
        struct DateHelper;
        
        impl HelperDef for DateHelper {
            fn call<'reg: 'rc, 'rc>(
                &self,
                h: &Helper<'rc>,
                _: &'reg Handlebars<'reg>,
                _: &'rc Context,
                _: &mut RenderContext<'reg, 'rc>,
                out: &mut dyn Output,
            ) -> std::result::Result<(), RenderError> {
                if let Some(date_str) = h.param(0).and_then(|v| v.value().as_str()) {
                    if let Ok(date) = chrono::NaiveDate::parse_from_str(date_str, "%Y-%m-%d") {
                        out.write(&date.format("%B %e, %Y").to_string())?;
                    } else if let Ok(datetime) = chrono::DateTime::parse_from_rfc3339(date_str) {
                        out.write(&datetime.format("%B %e, %Y").to_string())?;
                    } else {
                        out.write(date_str)?;
                    }
                }
                Ok(())
            }
        }
        
        // Helper for "safe" HTML (don't escape)
        struct SafeHelper;
        
        impl HelperDef for SafeHelper {
            fn call<'reg: 'rc, 'rc>(
                &self,
                h: &Helper<'rc>,
                _: &'reg Handlebars<'reg>,
                _: &'rc Context,
                _: &mut RenderContext<'reg, 'rc>,
                out: &mut dyn Output,
            ) -> std::result::Result<(), RenderError> {
                if let Some(value) = h.param(0) {
                    out.write(value.value().as_str().unwrap_or(""))?;
                }
                Ok(())
            }
        }
        
        handlebars.register_helper("eq", Box::new(EqHelper));
        handlebars.register_helper("neq", Box::new(NeqHelper));
        handlebars.register_helper("lowercase", Box::new(LowercaseHelper));
        handlebars.register_helper("date", Box::new(DateHelper));
        handlebars.register_helper("safe", Box::new(SafeHelper));
    }
    
    fn register_templates(handlebars: &mut Handlebars<'static>, dir: &Path) -> Result<()> {
        // Register base templates first
        let base_templates = vec![
            "base.html",
            "index.html",
            "post.html",
        ];
        
        for template_name in base_templates {
            let template_path = dir.join(template_name);
            if template_path.exists() {
                let content = fs::read_to_string(&template_path)?;
                handlebars.register_template_string(template_name, content)?;
                println!("Registered template: {}", template_name);
            }
        }
        
        // Register partials (files starting with underscore)
        for entry in walkdir::WalkDir::new(dir)
            .max_depth(2)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let path = entry.path();
            if path.is_file() && path.extension().map(|e| e == "html").unwrap_or(false) {
                let file_name = path.file_name()
                    .map(|s| s.to_string_lossy().to_string())
                    .unwrap_or_default();
                
                // Register partials (files starting with _)
                if file_name.starts_with('_') {
                    let content = fs::read_to_string(path)?;
                    // Remove leading _ and .html extension for partial name
                    let partial_name = file_name
                        .trim_start_matches('_')
                        .trim_end_matches(".html")
                        .to_string();
                    
                    handlebars.register_partial(&partial_name, content)?;
                    println!("Registered partial: {}", partial_name);
                }
            }
        }
        
        Ok(())
    }
    
    pub fn render<T: Serialize>(
        &self,
        template_name: &str,
        data: &T,
    ) -> Result<String> {
        self.handlebars.render(template_name, data)
            .map_err(RawSSGError::Render)
    }
    
    pub fn render_content(
        &self,
        content: &Content,
        context: &HashMap<String, serde_json::Value>,
    ) -> Result<String> {
        let mut data = serde_json::Map::new();
        
        // Add content data
        data.insert("content".to_string(), serde_json::Value::String(content.body.clone()));
        
        // Add frontmatter fields
        if let Some(title) = &content.frontmatter.title {
            data.insert("title".to_string(), serde_json::Value::String(title.clone()));
        }
        
        if let Some(date) = &content.frontmatter.date {
            data.insert("date".to_string(), serde_json::Value::String(date.clone()));
        }
        
        // Catatan: Hapus field author dan description karena tidak ada di struct Frontmatter
        // Field yang ada di Frontmatter berdasarkan error: title, slug, layout, template, date, dan 4 others
        
        // Add custom frontmatter fields
        for (key, value) in &content.frontmatter.custom {
            let json_value = serde_json::to_value(value).unwrap_or(serde_json::Value::Null);
            data.insert(key.clone(), json_value);
        }
        
        // Add config context
        data.insert("config".to_string(), serde_json::to_value(&self.config)?);
        
        // Add build time
        data.insert("build_time".to_string(), 
            serde_json::Value::String(chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string()));
        
        // Add current URL context
        if let Some(url) = context.get("current_url") {
            data.insert("current_url".to_string(), url.clone());
        }
        
        // Determine which template to use - menggunakan owned String untuk menghindari temporary references
        let template_name: String = content
            .frontmatter
            .template
            .clone()
            .or_else(|| content.frontmatter.layout.clone())
            .unwrap_or_else(|| self.config.template.default_layout.clone());
        
        // Fallback to base.html if the specified template doesn't exist
        let template_name = if self.handlebars.has_template(&template_name) {
            template_name
        } else {
            "base.html".to_string()
        };
        
        println!("Rendering with template: {}", template_name);
        self.render(&template_name, &serde_json::Value::Object(data))
    }
    
    pub fn minify_html(&self, html: &str) -> String {
        if !self.config.build.minify.html {
            return html.to_string();
        }
        
        let mut result = String::with_capacity(html.len());
        let mut in_whitespace = false;
        let mut in_pre = false;
        let mut in_script = false;
        let mut in_style = false;
        let mut in_comment = false;
        let mut in_tag = false;
        let mut in_quotes = false;
        let mut quote_char = '\0';
        
        let chars: Vec<char> = html.chars().collect();
        let mut i = 0;
        
        while i < chars.len() {
            let c = chars[i];
            
            // Handle HTML comments
            if !in_quotes && !in_script && !in_style && !in_pre {
                if c == '<' && i + 4 < chars.len() && &chars[i..i+4] == ['<', '!', '-', '-'] {
                    in_comment = true;
                    i += 4;
                    continue;
                }
                
                if in_comment && c == '-' && i + 3 < chars.len() && &chars[i..i+3] == ['-', '-', '>'] {
                    in_comment = false;
                    i += 3;
                    continue;
                }
                
                if in_comment {
                    i += 1;
                    continue;
                }
            }
            
            // Handle <pre>, <script>, and <style> tags
            if !in_quotes && c == '<' && i + 1 < chars.len() {
                let next = chars[i + 1];
                
                // Check for closing tags
                if next == '/' {
                    let mut tag_end = i + 2;
                    while tag_end < chars.len() && chars[tag_end].is_alphanumeric() {
                        tag_end += 1;
                    }
                    
                    if tag_end < chars.len() && chars[tag_end] == '>' {
                        let tag_name: String = chars[i+2..tag_end].iter().collect::<String>().to_lowercase();
                        if tag_name == "pre" {
                            in_pre = false;
                        } else if tag_name == "script" {
                            in_script = false;
                        } else if tag_name == "style" {
                            in_style = false;
                        }
                    }
                } 
                // Check for opening tags
                else if next.is_alphabetic() {
                    let mut tag_end = i + 1;
                    while tag_end < chars.len() && chars[tag_end].is_alphanumeric() {
                        tag_end += 1;
                    }
                    
                    if tag_end < chars.len() {
                        let tag_name: String = chars[i+1..tag_end].iter().collect::<String>().to_lowercase();
                        if tag_name == "pre" {
                            in_pre = true;
                        } else if tag_name == "script" {
                            in_script = true;
                        } else if tag_name == "style" {
                            in_style = true;
                        }
                    }
                }
            }
            
            if in_pre || in_script || in_style {
                result.push(c);
                i += 1;
                continue;
            }
            
            // Handle quotes inside tags
            if in_tag && (c == '"' || c == '\'') {
                if in_quotes && c == quote_char {
                    in_quotes = false;
                    quote_char = '\0';
                } else if !in_quotes {
                    in_quotes = true;
                    quote_char = c;
                }
                result.push(c);
                i += 1;
                continue;
            }
            
            // Handle tag boundaries
            if !in_quotes && c == '<' {
                in_tag = true;
            } else if !in_quotes && c == '>' {
                in_tag = false;
            }
            
            // Handle whitespace
            if c.is_whitespace() {
                if !in_quotes && !in_tag {
                    if !in_whitespace && result.chars().last().map(|last| !last.is_whitespace()).unwrap_or(true) {
                        result.push(' ');
                        in_whitespace = true;
                    }
                } else {
                    result.push(c);
                    in_whitespace = false;
                }
            } else {
                result.push(c);
                in_whitespace = false;
            }
            
            i += 1;
        }
        
        result.trim().to_string()
    }
}