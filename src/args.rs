use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// The root directory for the search.
    #[arg(short, long, default_value = ".")]
    pub path: PathBuf,

    /// Filter files by extension (e.g., txt, png).
    #[arg(short, long, value_delimiter = ',', num_args = 1..)]
    pub extension: Vec<String>,

    /// Filter files containing this pattern in their name.
    #[arg(short, long)]
    pub name_contains: Option<String>,

    /// Perform a case-insensitive search.
    #[arg(short, long, default_value_t = false)]
    pub ignore_case: bool,

    /// Delete the found files.
    #[arg(long, default_value_t = false)]
    pub delete: bool,

    /// Show files that would be deleted (simulation).
    #[arg(short, long, default_value_t = false)]
    pub dry_run: bool,
}
