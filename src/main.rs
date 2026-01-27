mod cli;
mod config;
mod builder;
mod template;
mod watcher;
mod frontmatter;
mod error;
use clap::Parser;
use std::process;
use crate::cli::{Cli, Commands};
use crate::config::RawConfig;
use crate::error::{RawSSGError, Result};
fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}
fn run() -> Result<()> {
    let cli = Cli::parse();
    if cli.verbose {
        tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();
    }
    match cli.command {
        Commands::Init { path, force } => init_project(path, force),
        Commands::Build { clean, watch, minify, drafts, config } => {
            build_project(config, clean, watch, minify, drafts)
        }
        Commands::Serve { port, watch, config } => serve_project(config, port, watch),
        Commands::New { content_type, title, draft, config } => {
            new_content(config, &content_type, &title, draft)
        }
        Commands::Clean { config } => clean_project(config),
        Commands::Validate { config } => validate_config(config),
    }
}
fn init_project(path: std::path::PathBuf, force: bool) -> Result<()> {
    let config_path = path.join(".rawfile");
    if config_path.exists() && !force {
        return Err(RawSSGError::Config(
                "Config file already exists. Use --force to overwrite.".into()
            ));
        }
        if !path.exists() {
            std::fs::create_dir_all(&path)?;
        }
        let config = RawConfig::default_config();
        config.save(&config_path)?;
        let content_dir = path.join("content");
        std::fs::create_dir_all(&content_dir)?;
        let index_content = r#"---
        title: Home
        slug: index
        date: 2026-01-01
        ---
        Welcome to your new RawSSG site!
        This is a sample page. You can edit this file in `content/index.rw`.
        Check out these links:
        - https://github.com/neuxdotdev
        - neuxdev1@gmail.com
        ## Features
        * Raw text support
        * Automatic link detection
        * Dark/light theme
        * Mobile responsive"#;
        std::fs::write(content_dir.join("index.rw"), index_content)?;
        let about_content = r#"---
        title: About
        slug: about
        date: 2026-01-02
        ---
        This is the about page.
        RawSSG is a minimalist static site generator that keeps your content raw and simple.
        Email: hello@example.com
        Website: https://example.com"#;
        std::fs::write(content_dir.join("about.rw"), about_content)?;
        println!("✅ RawSSG project initialized in {}", path.display());
        println!("📁 Directory structure created:");
        println!("   .rawfile       - Configuration file");
        println!("   content/       - Your content files");
        println!("   templates/     - HTML templates (auto-created on first build)");
        println!("   dist/          - Build output");
        println!("\nNext steps:");
        println!("1. Edit .rawfile to customize your site");
        println!("2. Add content to content/ directory");
        println!("3. Run `rawssg build` to generate your site");
        println!("4. Run `rawssg serve` to preview locally");
        Ok(())
    }
    fn build_project(
        config_path: std::path::PathBuf,
        clean: bool,
        watch: bool,
        minify: bool,
        drafts: bool,
    ) -> Result<()> {
    let config = RawConfig::load(&config_path)?;
    let mut config = config.clone();
    config.build.clean_build = clean;
    config.build.minify.html = minify;
    config.build.minify.css = minify;
    config.build.minify.js = minify;
    config.content.drafts = drafts;
    let builder = builder::Builder::new(config.clone())?;
    if watch {
        use crate::watcher::FileWatcher;
        println!("👀 Watching for changes... (Press Ctrl+C to stop)");
        builder.build()?;
        let mut watcher = FileWatcher::new(500)?;
        watcher.watch("content")?;
        watcher.watch("templates")?;
        watcher.watch(&config_path)?;
        watcher.wait_for_changes(|| {
                println!("🔄 Change detected, rebuilding...");
                if let Err(e) = builder.build() {
                    eprintln!("Build error: {}", e);
                }
            })?;
        } else {
        builder.build()?;
    }
    Ok(())
}
fn serve_project(
    config_path: std::path::PathBuf,
    port: u16,
    watch: bool,
) -> Result<()> {
use std::process::Command;
build_project(config_path.clone(), false, watch, false, false)?;
println!("🚀 Starting server on http://localhost:{}", port);
println!("📁 Serving from: dist/");
let python_cmd = if cfg!(target_os = "windows") {
    "python"
} else {
"python3"
};
let mut server = Command::new(python_cmd)
.args(["-m", "http.server", &port.to_string()])
.current_dir("dist")
.spawn()
.map_err(|e| RawSSGError::Build(format!("Failed to start server: {}", e)))?;
println!("✅ Server started. Press Ctrl+C to stop.");
let _ = server.wait();
Ok(())
}
fn new_content(
    config_path: std::path::PathBuf,
    content_type: &str,
    title: &str,
    draft: bool,
) -> Result<()> {
let config = RawConfig::load(&config_path)?;
let slug = title.to_lowercase()
.replace(' ', "-")
.replace(|c: char| !c.is_alphanumeric() && c != '-', "");
let content = match content_type {
    "page" | "post" => {
        let date = chrono::Local::now().format("%Y-%m-%d").to_string();
        format!(
            r#"---
            title: {}
            slug: {}
            date: {}
            draft: {}
            ---
            Start writing your {} here...
            https://example.com
            email@example.com"#,
            title, slug, date, draft, content_type
        )
    }
    _ => {
        return Err(RawSSGError::Config(
                format!("Unknown content type: {}", content_type)
            ));
        }
    };
    let filename = format!("{}.{}", slug,
        config.content.format.as_deref().unwrap_or("rw"));
    let content_dir = "content";
    std::fs::create_dir_all(content_dir)?;
    let filepath = std::path::Path::new(content_dir).join(&filename);
    std::fs::write(&filepath, content)?;
    println!("✅ Created new {}: {}", content_type, filepath.display());
    println!("   Edit the file and run `rawssg build` to publish.");
    Ok(())
}
fn clean_project(config_path: std::path::PathBuf) -> Result<()> {
    let config = RawConfig::load(&config_path)?;
    let output_dir = std::path::Path::new(&config.build.output_dir);
    if output_dir.exists() {
        std::fs::remove_dir_all(output_dir)?;
        println!("🧹 Cleaned output directory: {}", output_dir.display());
    } else {
    println!("📭 Output directory does not exist: {}", output_dir.display());
}
Ok(())
}
fn validate_config(config_path: std::path::PathBuf) -> Result<()> {
    let config = RawConfig::load(&config_path)?;
    println!("✅ Configuration is valid:");
    println!("   Version: {}", config.version);
    println!("   Title: {}", config.title);
    println!("   Output directory: {}", config.build.output_dir);
    println!("   Content format: {:?}", config.content.format);
    if config.build.strict {
        println!("   Strict mode: enabled");
    }
    Ok(())
}