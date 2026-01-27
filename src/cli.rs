use clap::{Parser, Subcommand, ValueEnum};

#[derive(Parser)]
#[command(
    name = "rawssg",
    version = env!("CARGO_PKG_VERSION"),
    author = "Neux.dev <neuxdev@gmail.com>",
    about = "A minimalist static site generator for raw text",
    long_about = r#"
RawSSG - Raw Static Site Generator
===================================
Build fast, simple websites from raw text files.
No frameworks, no build fatigue, just content.
    "#
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
    
    #[arg(
        short,
        long,
        global = true,
        help = "Enable verbose output",
        default_value_t = false
    )]
    pub verbose: bool,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Initialize a new RawSSG project
    Init {
        #[arg(
            default_value = ".",
            help = "Path to initialize project"
        )]
        path: std::path::PathBuf,
        
        #[arg(
            short,
            long,
            help = "Force overwrite existing config"
        )]
        force: bool,
    },
    
    /// Build the static site
    Build {
        #[arg(
            short,
            long,
            help = "Clean output directory before building"
        )]
        clean: bool,
        
        #[arg(
            short = 'w',
            long,
            help = "Watch for changes and rebuild"
        )]
        watch: bool,
        
        #[arg(
            short = 'm',
            long,
            help = "Minify HTML/CSS/JS output"
        )]
        minify: bool,
        
        #[arg(
            short = 'd',
            long,
            help = "Include draft content"
        )]
        drafts: bool,
        
        #[arg(
            short = 'c',
            long,
            default_value = ".rawfile",
            help = "Path to config file"
        )]
        config: std::path::PathBuf,
    },
    
    /// Serve the built site locally
    Serve {
        #[arg(
            short,
            long,
            default_value_t = 8000,
            help = "Port to serve on"
        )]
        port: u16,
        
        #[arg(
            short = 'w',
            long,
            help = "Watch for changes and rebuild"
        )]
        watch: bool,
        
        #[arg(
            short = 'c',
            long,
            default_value = ".rawfile",
            help = "Path to config file"
        )]
        config: std::path::PathBuf,
    },
    
    /// Create new content
    New {
        #[arg(
            value_enum,
            default_value = "page",
            help = "Type of content to create"
        )]
        content_type: ContentType,
        
        #[arg(
            help = "Title of the content"
        )]
        title: String,
        
        #[arg(
            short = 'd',
            long,
            help = "Create as draft"
        )]
        draft: bool,
        
        #[arg(
            short = 'c',
            long,
            default_value = ".rawfile",
            help = "Path to config file"
        )]
        config: std::path::PathBuf,
    },
    
    /// Clean output directory
    Clean {
        #[arg(
            short = 'c',
            long,
            default_value = ".rawfile",
            help = "Path to config file"
        )]
        config: std::path::PathBuf,
    },
    
    /// Validate configuration
    Validate {
        #[arg(
            short = 'c',
            long,
            default_value = ".rawfile",
            help = "Path to config file"
        )]
        config: std::path::PathBuf,
    },
    
    /// Show version information
    Version,
}

#[derive(ValueEnum, Clone)]
pub enum ContentType {
    Page,
    Post,
    Article,
    Note,
    Snippet,
}