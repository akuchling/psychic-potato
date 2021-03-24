use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub enum Action {
  /// Write tasks to the journal file
  Add {
    /// Task description text
    #[structopt()]
    text: String,
  },
  /// Remove an entry from the journal file
  Done {
    #[structopt()]
    position: usize,
  },
  /// List all tasks
  List,
}

#[derive(Debug, StructOpt)]
#[structopt(
    name = "Journal",
    about = "A command-line tool written in Rust",
)]
pub struct CommandLineArgs {
    #[structopt(subcommand)]
    pub action: Action,

    /// Use a different journal file.
    #[structopt(parse(from_os_str), short, long)]
    pub journal_file: Option<PathBuf>,
}
