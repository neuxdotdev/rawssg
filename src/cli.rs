use clap::{Parser, Subcommand};
use std::path::PathBuf;
#[derive(Parser)]
#[command(name = "rawssg")]
#[command(version = env!("CARGO_PKG_VERSION"))]
#[command(about = "A minimalist static site generator for raw text files", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
    #[arg(short, long, global = true)]
    pub verbose: bool,
}
#[derive(Subcommand)]
pub enum Commands {
    Init {
        #[arg(default_value = ".")]
        path: PathBuf,
        #[arg(short, long)]
        force: bool,
    },
    Build {
        #[arg(short, long)]
        clean: bool,
        #[arg(short, long)]
        watch: bool,
        #[arg(short = 'm', long)]
        minify: bool,
        #[arg(long)]
        drafts: bool,
        #[arg(long, default_value = ".rawfile")]
        config: PathBuf,
    },
    Serve {
        #[arg(short, long, default_value = "8080")]
        port: u16,
        #[arg(short, long)]
        watch: bool,
        #[arg(long, default_value = ".rawfile")]
        config: PathBuf,
    },
    New {
        #[arg(help = "Content type (page/post)")]
        content_type: String,
        #[arg(help = "Content title or slug")]
        title: String,
        #[arg(short, long)]
        draft: bool,
        #[arg(long, default_value = ".rawfile")]
        config: PathBuf,
    },
    Clean {
        #[arg(long, default_value = ".rawfile")]
        config: PathBuf,
    },
    Validate {
        #[arg(default_value = ".rawfile")]
        config: PathBuf,
    },
}