mod cli;
mod config;
mod builder;
mod template;
mod watcher;
mod frontmatter;
mod error;
use std::path::Path;
use clap::Parser;
use std::process;
use crate::cli::{Cli, Commands, ContentType};
use crate::error::{RawSSGError, Result};
fn main() {
    if let Err(e) = run() {
        eprintln!("❌ Error: {}", e);
        if cfg!(debug_assertions) {
            eprintln!("🔍 Backtrace: {:?}", e);
        }
        process::exit(1);
    }
}

fn run() -> Result<()> {
    let cli = Cli::parse();
    
    // Setup logging
    setup_logging(cli.verbose);
    
    // Process command
    process_command(cli.command)
}

fn setup_logging(verbose: bool) {
    if verbose || cfg!(debug_assertions) {
        tracing_subscriber::fmt()
            .with_max_level(tracing::Level::DEBUG)
            .with_target(true)
            .with_line_number(true)
            .init();
    } else {
        tracing_subscriber::fmt()
            .with_max_level(tracing::Level::INFO)
            .without_time()
            .with_target(false)
            .init();
    }
}

fn process_command(command: Commands) -> Result<()> {
    match command {
        Commands::Init { path, force } => {
            init_project(path, force)
        }
        Commands::Build { 
            clean, 
            watch, 
            minify, 
            drafts, 
            config 
        } => {
            build_project(config, clean, watch, minify, drafts)
        }
        Commands::Serve { 
            port, 
            watch, 
            config 
        } => {
            serve_project(config, port, watch)
        }
        Commands::New { content_type, title, draft, config } => {
    let content_type_str = match content_type {
        ContentType::Page => "page",
        ContentType::Post => "post",
        ContentType::Article => "article",
        ContentType::Note => "note",
        ContentType::Snippet => "snippet",
    };

    new_content(config, content_type_str, &title, draft)
}

        Commands::Clean { config } => {
            clean_project(config)
        }
        Commands::Validate { config } => {
            validate_config(config)
        }
        Commands::Version => {
            print_version();
            Ok(())
        }
    }
}

fn init_project(path: std::path::PathBuf, force: bool) -> Result<()> {
    use crate::config::RawConfig;
    
    tracing::info!("Initializing new RawSSG project at {}", path.display());
    
    let config_path = path.join(".rawfile");
    if config_path.exists() && !force {
        return Err(RawSSGError::Config(
            "Config file already exists. Use --force to overwrite.".into()
        ));
    }
    
    // Create directory structure
    create_project_structure(&path)?;
    
    // Create default config
    let config = RawConfig::default_config();
    config.save(&config_path)?;
    
    // Create sample content
    create_sample_content(&path)?;
    
    // Copy default templates
    copy_default_templates(&path)?;
    
    println!("✅ RawSSG project initialized successfully!");
    println!("📁 Project structure created at: {}", path.display());
    println!("\n🎉 Next steps:");
    println!("   1. cd {}", path.display());
    println!("   2. Edit .rawfile to customize your site");
    println!("   3. Add content to content/ directory");
    println!("   4. Run `rawssg build` to generate your site");
    println!("   5. Run `rawssg serve` to preview locally");
    
    Ok(())
}

fn create_project_structure(path: &std::path::Path) -> Result<()> {
    let dirs = vec![
        "content",
        "templates",
        "templates/partials",
        "templates/static/css",
        "templates/static/js",
        "templates/static/images",
        "dist",
        "assets",
    ];
    
    for dir in dirs {
        let dir_path = path.join(dir);
        if !dir_path.exists() {
            std::fs::create_dir_all(&dir_path)?;
            tracing::debug!("Created directory: {}", dir_path.display());
        }
    }
    
    Ok(())
}

fn create_sample_content(project_path: &std::path::Path) -> Result<()> {
    let content_dir = project_path.join("content");
    
    let index_content = r#"---
title: Home
slug: index
date: 2024-01-01
layout: base
---
# Welcome to RawSSG! ✨

This is a sample page. You can edit this file in `content/index.rw`.

## Features
* 📝 **Raw text support** - Write in plain text
* 🔗 **Automatic link detection** - URLs and emails become clickable
* 🌓 **Dark/light theme** - Auto-detection with manual toggle
* 📱 **Mobile responsive** - Works on all devices
* ⚡ **Fast & minimal** - No JavaScript framework bloat

## Quick Links
- https://github.com/neuxdotdev/rawssg
- hello@example.com
- https://rawssg.neux.dev

## Getting Started
1. Edit your content in the `content/` directory
2. Customize templates in `templates/`
3. Run `rawssg build` to generate your site
4. Run `rawssg serve` to preview locally

Happy building! 🚀
"#;
    
    let about_content = r#"---
title: About
slug: about
date: 2024-01-02
layout: base
---
# About RawSSG

RawSSG is a minimalist static site generator that keeps your content **raw and simple**.

## Philosophy
- **Simplicity over complexity** - Write in plain text, get clean HTML
- **Own your content** - No vendor lock-in, just files
- **Performance by default** - No unnecessary JavaScript
- **Accessibility first** - Semantic HTML by default

## Why RawSSG?
1. **No build step fatigue** - Just write and build
2. **Portable content** - Your content is just text files
3. **Fast loading** - Static HTML with minimal CSS/JS
4. **Easy to customize** - Simple Handlebars templates

## Contact
Email: hello@example.com
GitHub: https://github.com/neuxdotdev
Website: https://neux.dev

---
*Built with ❤️ using RawSSG*
"#;
    
    std::fs::write(content_dir.join("index.rw"), index_content)?;
    std::fs::write(content_dir.join("about.rw"), about_content)?;
    
    tracing::debug!("Created sample content files");
    Ok(())
}

fn copy_default_templates(project_path: &std::path::Path) -> Result<()> {
    // Copy template files from crate's built-in templates
    // (This would use include_dir! or similar in real implementation)
    
    let template_dir = project_path.join("templates");
    
    // Create README for templates
    let readme = r#"# RawSSG Templates

This directory contains your site templates.

## Structure
- `base.html` - Main layout template
- `partials/` - Reusable template parts
- `static/` - CSS, JS, images

## Available Partials
- `sidebar` - Navigation sidebar
- `content` - Main content area
- `footer` - Site footer

## Template Variables
- `{{title}}` - Page title
- `{{content}}` - Main content
- `{{config}}` - Site configuration
- `{{current_url}}` - Current page URL

## Helpers
- `{{date format}}` - Format dates
- `{{eq a b}}` - Equality check
- `{{lowercase str}}` - Convert to lowercase
"#;
    
    std::fs::write(template_dir.join("README.md"), readme)?;
    
    tracing::info!("Default templates README created");
    Ok(())
}

fn build_project(
    config_path: std::path::PathBuf,
    clean: bool,
    watch: bool,
    minify: bool,
    drafts: bool,
) -> Result<()> {
    use crate::config::RawConfig;
    use crate::builder::Builder;
    
    tracing::info!("Building site...");
    
    let config = RawConfig::load(&config_path)?;
    let mut config = config.clone();
    
    // Apply build options
    config.build.clean_build = clean;
    config.build.minify.html = minify;
    config.build.minify.css = minify;
    config.build.minify.js = minify;
    config.content.drafts = drafts;
    
    let builder = Builder::new(config.clone())?;
    
    if watch {
        use crate::watcher::FileWatcher;
        
        println!("👀 Watching for changes... (Press Ctrl+C to stop)");
        println!("📁 Watching: content/, templates/, {}", config_path.display());
        
        // Initial build
        builder.build()?;
        
        // Setup watcher
        let mut watcher = FileWatcher::new(500)?;
        watcher.watch("content")?;
        watcher.watch("templates")?;
        watcher.watch(&config_path)?;
        
        // Watch loop
        watcher.wait_for_changes(|| {
            println!("\n🔄 Change detected, rebuilding...");
            let start_time = std::time::Instant::now();
            
            if let Err(e) = builder.build() {
                eprintln!("❌ Build error: {}", e);
                println!("💡 Tip: Check template syntax or content format");
            } else {
                let duration = start_time.elapsed();
                println!("✅ Rebuild completed in {:.2?}", duration);
            }
        })?;
    } else {
        // Single build
        let start_time = std::time::Instant::now();
        builder.build()?;
        let duration = start_time.elapsed();
        
        println!("✅ Build completed in {:.2?}", duration);
        
        // Show build stats
        let dist_path = std::path::Path::new(&config.build.output_dir);
        if dist_path.exists() {
            let count = std::fs::read_dir(dist_path)?
                .filter_map(|e| e.ok())
                .filter(|e| e.path().extension().map(|ext| ext == "html").unwrap_or(false))
                .count();
            
            println!("📄 Generated {} HTML pages", count);
        }
    }
    
    Ok(())
}

fn serve_project(
    config_path: std::path::PathBuf,
    port: u16,
    watch: bool,
) -> Result<()> {
    use std::process::Command;
    
    println!("🚀 Starting RawSSG development server");
    println!("🌐 http://localhost:{}", port);
    println!("📁 Serving from: dist/");
    
    // Build first
    build_project(config_path.clone(), false, watch, false, false)?;
    
    // Determine Python command
    let python_cmd = if cfg!(target_os = "windows") {
        "python"
    } else {
        "python3"
    };
    
    // Check if Python is available
    let python_check = Command::new(python_cmd)
        .arg("--version")
        .output();
    
    if python_check.is_err() {
        tracing::warn!("Python not found, trying alternative server...");
        
        // Fallback to built-in server if Python not available
        return serve_with_builtin(port);
    }
    
    // Start Python HTTP server
    let mut server = Command::new(python_cmd)
        .args(["-m", "http.server", &port.to_string()])
        .current_dir("dist")
        .spawn()
        .map_err(|e| RawSSGError::Build(format!("Failed to start server: {}", e)))?;
    
    println!("✅ Server started successfully!");
    println!("📢 Press Ctrl+C to stop the server");
    println!("\n💡 Tip: Open http://localhost:{} in your browser", port);
    
    let _ = server.wait();
    Ok(())
}

fn serve_with_builtin(port: u16) -> Result<()> {
    // Simple built-in server fallback
    println!("⚡ Using built-in file server");
    println!("📁 Serving files from: dist/");
    println!("🌐 Open in browser: http://localhost:{}", port);
    
    // This would be replaced with actual server implementation
    println!("🔧 Built-in server coming soon!");
    println!("💡 For now, please install Python or use another static file server");
    
    Ok(())
}

fn new_content(
    config_path: std::path::PathBuf,
    content_type: &str,
    title: &str,
    draft: bool,
) -> Result<()> {
    use crate::config::RawConfig;
    use chrono::Local;
    
    tracing::info!("Creating new {}: '{}'", content_type, title);
    
    let config = RawConfig::load(&config_path)?;
    
    // Generate slug from title
    let slug = title.to_lowercase()
        .trim()
        .replace(' ', "-")
        .replace(|c: char| !c.is_alphanumeric() && c != '-', "");
    
    // Format current date
    let date = Local::now().format("%Y-%m-%d").to_string();
    
    // Create content based on type
    let content = match content_type.to_lowercase().as_str() {
        "page" | "post" | "article" => {
            format!(r#"---
title: {}
slug: {}
date: {}
layout: base
draft: {}
tags: []
category: {}
---
# {}

Start writing your {} here...

## Introduction
Write your introduction here.

## Main Content
Add your main content here.

## Conclusion
Wrap up your {} here.

---
*Published on {}*
"#,
                title, slug, date, draft, content_type, title, content_type, content_type, date
            )
        }
        "note" | "snippet" => {
            format!(r#"---
title: {}
slug: {}
date: {}
layout: base
draft: {}
type: note
---
# {}

Quick note: {}

---
*Note created on {}*
"#,
                title, slug, date, draft, title, title, date
            )
        }
        _ => {
            return Err(RawSSGError::Config(
                format!("Unknown content type: {}. Use 'page', 'post', or 'note'", content_type)
            ));
        }
    };
    
    // Determine file extension
    let extension = config.content.format.as_deref().unwrap_or("rw");
    let filename = format!("{}.{}", slug, extension);
    
    // Ensure content directory exists
    let content_dir = std::path::Path::new("content");
    std::fs::create_dir_all(content_dir)?;
    
    // Write file
    let filepath = content_dir.join(&filename);
    std::fs::write(&filepath, content)?;
    
    println!("✅ Created new {}:", content_type);
    println!("   📄 File: {}", filepath.display());
    println!("   🆔 Slug: {}", slug);
    println!("   📅 Date: {}", date);
    println!("   📝 Draft: {}", draft);
    println!("\n💡 Next steps:");
    println!("   1. Edit {}", filepath.display());
    println!("   2. Add your content");
    println!("   3. Run `rawssg build` to publish");
    
    Ok(())
}

fn clean_project(config_path: std::path::PathBuf) -> Result<()> {
    use crate::config::RawConfig;
    
    tracing::info!("Cleaning project...");
    
    let config = RawConfig::load(&config_path)?;
    let output_dir = std::path::Path::new(&config.build.output_dir);
    
    if output_dir.exists() {
        let count = count_files_in_dir(output_dir)?;
        
        std::fs::remove_dir_all(output_dir)?;
        println!("🧹 Cleaned output directory: {}", output_dir.display());
        println!("📄 Removed {} files/folders", count);
        
        // Also clean cache if exists
        let cache_dir = std::path::Path::new(".rawssg-cache");
        if cache_dir.exists() {
            std::fs::remove_dir_all(cache_dir)?;
            println!("🗑️  Cleared build cache");
        }
    } else {
        println!("📭 Output directory does not exist: {}", output_dir.display());
    }
    
    Ok(())
}

fn count_files_in_dir(dir: &std::path::Path) -> Result<usize> {
    let mut count = 0;
    if dir.is_dir() {
        for entry in std::fs::read_dir(dir)? {
            let _ = entry?;
            count += 1;
        }
    }
    Ok(count)
}

fn validate_config(config_path: std::path::PathBuf) -> Result<()> {
    use crate::config::RawConfig;
    
    tracing::info!("Validating configuration...");
    
    let config = RawConfig::load(&config_path)?;
    
    println!("✅ Configuration is valid!");
    println!("\n📋 Configuration Summary:");
    println!("   📦 Version: {}", config.version);
    println!("   🏷️  Title: {}", config.title);
    if let Some(desc) = &config.description {
        println!("   📝 Description: {}", desc);
    }
    println!("   📂 Output: {}", config.build.output_dir);
    println!("   📄 Content format: .{}", config.content.format.as_deref().unwrap_or("rw"));
    println!("   🎨 Template dir: {}", config.template.dir);
    println!("   📱 Default layout: {}", config.template.default_layout);
    
    // Check directories
    println!("\n🔍 Directory Status:");
    
    let dirs_to_check = vec![
        ("content/", Path::new("content").exists()),
        ("templates/", Path::new(&config.template.dir).exists()),
        (&config.build.output_dir, Path::new(&config.build.output_dir).exists()),
    ];
    
    for (dir_name, exists) in dirs_to_check {
        let status = if exists { "✅" } else { "❌" };
        println!("   {} {}", status, dir_name);
    }
    
    // Validate templates if they exist
    let template_dir = Path::new(&config.template.dir);
    if template_dir.exists() {
        let template_files = vec![
            "base.html",
            "partials/_sidebar.html",
            "partials/_content.html",
            "partials/_footer.html",
        ];
        
        println!("\n🎨 Template Status:");
        for file in template_files {
            let path = template_dir.join(file);
            let status = if path.exists() { "✅" } else { "⚠️ " };
            println!("   {} {}", status, file);
        }
    }
    
    if config.build.strict {
        println!("\n🔒 Strict mode: Enabled");
        println!("   💡 All warnings will be treated as errors");
    }
    
    Ok(())
}

fn print_version() {
    println!("RawSSG v{}", env!("CARGO_PKG_VERSION"));
    println!("A minimalist static site generator for raw text");
    println!("📦 https://github.com/neuxdotdev/rawssg");
}