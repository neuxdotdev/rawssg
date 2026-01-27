use handlebars::{Context, Handlebars, Helper, HelperDef, Output, RenderContext, RenderError};
use serde::Serialize;
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use crate::config::RawConfig;
use crate::error::Result;
use crate::frontmatter::Content;
use crate::error::RawSSGError;

pub struct TemplateEngine {
    handlebars: Handlebars<'static>,
    config: RawConfig,
}

impl TemplateEngine {
    pub fn new(config: &RawConfig) -> Result<Self> {
        let mut handlebars = Handlebars::new();
        Self::register_helpers(&mut handlebars);
        let template_dir = Path::new(&config.template.dir);
        
        if template_dir.exists() {
            Self::register_templates(&mut handlebars, template_dir)?;
        } else {
            Self::create_default_templates(template_dir)?;
            Self::register_templates(&mut handlebars, template_dir)?;
        }
        
        Ok(Self {
            handlebars,
            config: config.clone(),
        })
    }
    
    fn register_helpers(handlebars: &mut Handlebars<'static>) {
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
                let a = h.param(0)
                    .and_then(|v| v.value().as_i64())
                    .unwrap_or(0);
                
                let b = h.param(1)
                    .and_then(|v| v.value().as_i64())
                    .unwrap_or(0);
                
                out.write(&(a == b).to_string())?;
                Ok(())
            }
        }
        
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
                let a = h.param(0)
                    .and_then(|v| v.value().as_i64())
                    .unwrap_or(0);
                
                let b = h.param(1)
                    .and_then(|v| v.value().as_i64())
                    .unwrap_or(0);
                
                out.write(&(a != b).to_string())?;
                Ok(())
            }
        }
        
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
                    } else {
                        out.write(date_str)?;
                    }
                }
                Ok(())
            }
        }
        
        handlebars.register_helper("eq", Box::new(EqHelper));
        handlebars.register_helper("neq", Box::new(NeqHelper));
        handlebars.register_helper("lowercase", Box::new(LowercaseHelper));
        handlebars.register_helper("date", Box::new(DateHelper));
    }
    
    fn register_templates(handlebars: &mut Handlebars<'static>, dir: &Path) -> Result<()> {
        for entry in walkdir::WalkDir::new(dir)
            .follow_links(true)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let path = entry.path();
            if path.is_file() && path.extension().map(|e| e == "html").unwrap_or(false) {
                let content = fs::read_to_string(path).map_err(RawSSGError::Io)?;
                
                let file_name = path.file_name()
                    .map(|s| s.to_string_lossy().to_string())
                    .unwrap_or_default();
                
                if path.starts_with(dir.join("partials")) {
                    let partial_name = file_name.trim_start_matches('_').trim_end_matches(".html");
                    handlebars.register_partial(partial_name, content)
                        .map_err(RawSSGError::Template)?;
                } else {
                    let template_name = path
                        .strip_prefix(dir)
                        .map_err(RawSSGError::StripPrefix)?
                        .to_string_lossy()
                        .replace('\\', "/");
                    
                    handlebars.register_template_string(&template_name, content)
                        .map_err(RawSSGError::Template)?;
                }
            }
        }
        Ok(())
    }
    
    fn create_default_templates(dir: &Path) -> Result<()> {
        fs::create_dir_all(dir)?;
        
        let base_template = r#"<!DOCTYPE html>
<html lang="en" data-theme="light">
<head>
    <meta charset="utf-8">
    <meta name="viewport" content="width=device-width, initial-scale=1">
    <meta name="color-scheme" content="light dark">
    <title>{{#if title}}{{title}} - {{/if}}{{config.title}}</title>
    {{#if config.description}}<meta name="description" content="{{config.description}}">{{/if}}
    {{#if config.favicon}}<link rel="icon" href="{{config.favicon}}">{{/if}}
    <link rel="stylesheet" href="/css/styles.css">
    <script src="/js/scripts.js" defer></script>
    {{#if config.custom.head}}{{config.custom.head | safe}}{{/if}}
</head>
<body>
    <div class="container">
        {{> sidebar }}
        
        <main class="content">
            {{> content }}
        </main>
    </div>
    
    {{> footer }}
    
    <script>
        // Theme switching
        const theme = localStorage.getItem('theme') || 
                     (window.matchMedia('(prefers-color-scheme: dark)').matches ? 'dark' : 'light');
        document.documentElement.setAttribute('data-theme', theme);
        
        // Link auto-detection
        document.querySelectorAll('.raw-content').forEach(el => {
            const text = el.textContent;
            el.innerHTML = text
                .replace(
                    /(https?:\/\/[^\s]+)/g,
                    '<a href="$1" class="rw-link" target="_blank" rel="noopener">$1</a>'
                )
                .replace(
                    /([a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-z]{2,})/g,
                    '<a href="mailto:$1" class="rw-link">$1</a>'
                );
        });
    </script>
</body>
</html>"#;
        
        fs::write(dir.join("base.html"), base_template)?;
        
        let sidebar = r#"<nav class="sidebar">
    <div class="sidebar-header">
        <h2><a href="/">{{config.title}}</a></h2>
        <button class="sidebar-toggle" aria-label="Toggle menu">☰</button>
    </div>
    <ul class="sidebar-menu">
        {{#each config.nav}}
        <li>
            <a href="{{url}}" class="sidebar-link {{#if (eq url ../current_url)}}active{{/if}}">
                {{title}}
            </a>
            {{#if items}}
            <ul class="sidebar-submenu">
                {{#each items}}
                <li><a href="{{url}}">{{title}}</a></li>
                {{/each}}
            </ul>
            {{/if}}
        </li>
        {{/each}}
    </ul>
</nav>"#;
        
        fs::write(dir.join("_sidebar.html"), sidebar)?;
        
        let content = r#"{{#if title}}<h1 class="page-title">{{title}}</h1>{{/if}}
<div class="raw-content">
{{content}}
</div>
{{#if date}}<div class="meta">Published on {{date date}}</div>{{/if}}"#;
        
        fs::write(dir.join("_content.html"), content)?;
        
        let footer = r#"<footer class="footer">
    <div class="footer-content">
        {{#if config.footer.text}}<p>{{config.footer.text}}</p>{{/if}}
        {{#if config.footer.links}}
        <div class="footer-links">
            {{#each config.footer.links}}
            <a href="{{url}}">{{title}}</a>
            {{/each}}
        </div>
        {{/if}}
        {{#if config.footer.copyright}}
        <div class="copyright">{{config.footer.copyright}}</div>
        {{/if}}
        <div class="build-info">
            Built with <a href="https://github.com/neuxdotdev/rawssg">RawSSG</a> 
            at {{build_time}}
        </div>
    </div>
</footer>"#;
        
        fs::write(dir.join("_footer.html"), footer)?;
        
        let css_dir = dir.join("static/css");
        fs::create_dir_all(&css_dir)?;
        
        let css = r#"/* RawSSG Default Theme */
:root {
    --bg-light: #ffffff;
    --text-light: #24292f;
    --bg-dark: #0d1117;
    --text-dark: #c9d1d9;
    --border-light: #d0d7de;
    --border-dark: #30363d;
    --primary: #58a6ff;
    --secondary: #8b949e;
}

[data-theme="light"] {
    --bg: var(--bg-light);
    --text: var(--text-light);
    --border: var(--border-light);
}

[data-theme="dark"] {
    --bg: var(--bg-dark);
    --text: var(--text-dark);
    --border: var(--border-dark);
}

* {
    box-sizing: border-box;
    margin: 0;
    padding: 0;
}

body {
    font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen, Ubuntu, sans-serif;
    line-height: 1.6;
    color: var(--text);
    background: var(--bg);
    transition: background-color 0.3s, color 0.3s;
}

.container {
    display: flex;
    min-height: 100vh;
}

.sidebar {
    width: 250px;
    background: var(--bg);
    border-right: 1px solid var(--border);
    padding: 1.5rem;
    position: sticky;
    top: 0;
    height: 100vh;
    overflow-y: auto;
}

.sidebar-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 2rem;
}

.sidebar-header h2 {
    font-size: 1.25rem;
    font-weight: 600;
}

.sidebar-toggle {
    display: none;
    background: none;
    border: 1px solid var(--border);
    border-radius: 4px;
    padding: 0.5rem;
    cursor: pointer;
    color: var(--text);
}

.sidebar-menu {
    list-style: none;
}

.sidebar-link {
    display: block;
    padding: 0.5rem 0;
    color: var(--text);
    text-decoration: none;
    transition: color 0.2s;
}

.sidebar-link:hover,
.sidebar-link.active {
    color: var(--primary);
}

.content {
    flex: 1;
    padding: 2rem;
    max-width: 800px;
    margin: 0 auto;
}

.raw-content {
    font-family: 'SF Mono', 'Monaco', 'Inconsolata', 'Fira Code', monospace;
    font-size: 14px;
    line-height: 1.8;
    white-space: pre-wrap;
    word-wrap: break-word;
}

.rw-link {
    color: var(--primary);
    text-decoration: none;
    border-bottom: 1px dashed var(--primary);
}

.rw-link:hover {
    text-decoration: underline;
}

.footer {
    margin-top: 4rem;
    padding: 2rem 0;
    border-top: 1px solid var(--border);
    text-align: center;
    color: var(--secondary);
    font-size: 0.875rem;
}

.footer-links {
    display: flex;
    justify-content: center;
    gap: 1rem;
    margin: 1rem 0;
}

.footer-links a {
    color: var(--secondary);
    text-decoration: none;
}

.footer-links a:hover {
    color: var(--primary);
}

.build-info {
    margin-top: 1rem;
    font-size: 0.75rem;
    opacity: 0.7;
}

@media (max-width: 768px) {
    .container {
        flex-direction: column;
    }
    
    .sidebar {
        position: fixed;
        left: -250px;
        top: 0;
        height: 100vh;
        z-index: 1000;
        transition: left 0.3s;
    }
    
    .sidebar.active {
        left: 0;
    }
    
    .sidebar-toggle {
        display: block;
    }
    
    .content {
        padding: 1rem;
    }
}"#;
        
        fs::write(css_dir.join("styles.css"), css)?;
        
        let js_dir = dir.join("static/js");
        fs::create_dir_all(&js_dir)?;
        
        let js = r#"// Theme toggle
const themeToggle = document.createElement('button');
themeToggle.className = 'theme-toggle';
themeToggle.innerHTML = '🌓';
themeToggle.title = 'Toggle theme';
themeToggle.addEventListener('click', () => {
    const currentTheme = document.documentElement.getAttribute('data-theme');
    const newTheme = currentTheme === 'light' ? 'dark' : 'light';
    document.documentElement.setAttribute('data-theme', newTheme);
    localStorage.setItem('theme', newTheme);
});

// Mobile menu toggle
const sidebarToggle = document.querySelector('.sidebar-toggle');
if (sidebarToggle) {
    sidebarToggle.addEventListener('click', () => {
        document.querySelector('.sidebar').classList.toggle('active');
    });
}

// Close sidebar when clicking outside on mobile
document.addEventListener('click', (e) => {
    if (window.innerWidth <= 768) {
        const sidebar = document.querySelector('.sidebar');
        const toggle = document.querySelector('.sidebar-toggle');
        if (sidebar && toggle && 
            !sidebar.contains(e.target) && 
            !toggle.contains(e.target)) {
            sidebar.classList.remove('active');
        }
    }
});

// Add theme toggle to sidebar header
const sidebarHeader = document.querySelector('.sidebar-header');
if (sidebarHeader) {
    sidebarHeader.appendChild(themeToggle);
}"#;
        
        fs::write(js_dir.join("scripts.js"), js)?;
        
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
        
        if let Some(title) = &content.frontmatter.title {
            data.insert("title".to_string(), serde_json::Value::String(title.clone()));
        }
        
        data.insert("content".to_string(), serde_json::Value::String(content.body.clone()));
        
        for (key, value) in &content.frontmatter.custom {
            let json_value = serde_json::to_value(value).unwrap_or(serde_json::Value::Null);
            data.insert(key.clone(), json_value);
        }
        
        for (key, value) in context {
            data.insert(key.clone(), value.clone());
        }
        
        let template_name = content
            .frontmatter
            .template
            .as_deref()
            .or_else(|| content.frontmatter.layout.as_deref())
            .unwrap_or(&self.config.template.default_layout);
        
        self.render(template_name, &serde_json::Value::Object(data))
    }
    
    pub fn minify_html(&self, html: &str) -> String {
        if self.config.build.minify.html {
            let mut result = String::new();
            let mut in_tag = false;
            let mut in_quotes = false;
            let mut last_char = '\0';
            
            for c in html.chars() {
                match c {
                    '<' => {
                        in_tag = true;
                        result.push(c);
                    }
                    '>' => {
                        in_tag = false;
                        result.push(c);
                    }
                    '"' => {
                        in_quotes = !in_quotes;
                        result.push(c);
                    }
                    ' ' | '\t' | '\n' | '\r' => {
                        if in_tag && !in_quotes {
                            if last_char != ' ' && last_char != '>' && last_char != '<' {
                                result.push(' ');
                            }
                        } else if !in_tag {
                            if last_char != ' ' && last_char != '\n' {
                                result.push(c);
                            }
                        }
                    }
                    _ => {
                        result.push(c);
                    }
                }
                last_char = c;
            }
            
            let mut final_result = String::new();
            let mut last_was_space = false;
            
            for c in result.chars() {
                if c.is_whitespace() {
                    if !last_was_space {
                        final_result.push(' ');
                        last_was_space = true;
                    }
                } else {
                    final_result.push(c);
                    last_was_space = false;
                }
            }
            
            final_result.trim().to_string()
        } else {
            html.to_string()
        }
    }
}