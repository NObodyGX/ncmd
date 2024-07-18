mod cmd;
mod utils;

use clap::{Args, Parser, Subcommand};



#[derive(Debug, Parser)]
#[command(name = "ncmd")]
#[command(about = "A set of tools for files and dirs deal", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Args)]
struct TidyArgs {
    idir: String,
    #[arg(short, long, default_value_t=1)]
    recursion: u8,
}

#[derive(Debug, Args)]
struct RenameArgs {
    idir: String,
    /// output name, sp for: {num}, {date}, {time}, {file}
    #[arg(short, long)]
    name: Option<String>,
    /// the renames file suffix, case-insensitive
    #[arg(short, long)]
    filter: Option<String>,
    /// {num} start index
    #[arg(long="start", default_value_t=1)]
    start: i8,
    /// {num} increase index
    #[arg(long="gap", default_value_t=1)]
    gap: i8,
    #[arg(short, long)]
    preview: bool,
    #[arg(short, long)]
    recursion: bool,
    #[arg(long)]
    ignore_case: bool
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Remove trash files and dirs in dir.
    Tidy(TidyArgs),
    /// rename files in dir.
    Rename(RenameArgs),
}

fn main() {
    let args = Cli::parse();

    match args.command {
        Commands::Tidy(tidy) => {
            let x = tidy.idir;
            let xx = tidy.recursion;
            println!("Tidy {x} {xx}");
        }
        Commands::Rename(rename) => {
            let idir = rename.idir;
            let name = match rename.name {
                Some(name) => {name},
                None => "{num}".to_string(),
            }; 
            let r = rename.recursion;
            let p = rename.preview;
            let mut v: Vec<String> = Vec::new();
            let suffix = match rename.filter {
                Some(v) => {v},
                None => "jpg".to_string(),
            };
            let start = rename.start;
            let gap = rename.gap;
            v.push("jpg".to_string());
            v.push("png".to_string());
            cmd::g_rename(&idir, &suffix, name, start, gap, r, p);
        }
    }

    // Continued program logic goes here...
}
