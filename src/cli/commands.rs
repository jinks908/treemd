use clap::{Parser, ValueEnum};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "treemd")]
#[command(version, about = "A markdown navigator with tree-based structural navigation", long_about = None)]
pub struct Cli {
    /// Markdown file to view
    pub file: PathBuf,

    #[command(subcommand)]
    pub command: Option<Command>,

    /// List all headings (non-interactive)
    #[arg(short = 'l', long = "list")]
    pub list: bool,

    /// Show heading tree structure (non-interactive)
    #[arg(long = "tree")]
    pub tree: bool,

    /// Filter headings by text
    #[arg(long = "filter")]
    pub filter: Option<String>,

    /// Show only headings at specific level (1-6)
    #[arg(short = 'L', long = "level")]
    pub level: Option<usize>,

    /// Output format
    #[arg(short = 'o', long = "output", default_value = "plain")]
    pub output: OutputFormat,

    /// Extract specific section by heading name
    #[arg(short = 's', long = "section")]
    pub section: Option<String>,

    /// Count headings by level
    #[arg(long = "count")]
    pub count: bool,
}

#[derive(Debug, clap::Subcommand)]
pub enum Command {
    /// Show heading at specific line number
    AtLine {
        /// Line number
        line: usize,
    },
}

#[derive(Debug, Clone, ValueEnum)]
pub enum OutputFormat {
    /// Plain text output
    Plain,
    /// JSON output
    Json,
    /// Tree format with box-drawing
    Tree,
}
