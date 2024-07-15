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
    /// name special with {num}, {date}, {time}, {timestamp}
    #[arg(short, long)]
    name: Option<String>,
    #[arg(long="start", default_value_t=1)]
    start: u8,
    #[arg(long="gap", default_value_t=1)]
    gap: u8,
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
            let r = rename.recursion;
            let p = rename.preview;
            let name = match rename.name {
                Some(name) => {name},
                None => "".to_string(),
            }; 
            println!("Rename {idir}, name: {name}, {r} {p}");
        }
    }

    // Continued program logic goes here...
}
