use clap::{Parser, Subcommand};

mod commands;

#[derive(Parser)]
#[command(name = "vreq")]
#[command(about = "Python requirements helper for virtualenvs")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Req {
        #[command(subcommand)]
        action: ReqAction,
    },
}

#[derive(Subcommand)]
enum ReqAction {
    // generate requirements file
    #[command(alias = "gen", alias = "g")]
    Generate {
        // output file (default: requirements.txt)
        #[arg(short, long, default_value = "requirements.txt")]
        output: String,

        // include all dependencies
        #[arg(long)]
        all: bool,
    },

    // Install dependencies from requirements file
    #[command(alias = "s")]
    Sync {
        // input file arguement (default: requirements.txt)
        #[arg(short, long, default_value = "requirements.txt")]
        input: String,

        #[arg(long)]
        all: bool,
    },
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Req { action } => match action {
            ReqAction::Generate { output, all } => {
                commands::req::generate(&output, all)?
            }
            ReqAction::Sync { input, all } => {
                commands::req::sync(&input, all)?
            }
        },
    }

    Ok(())
}