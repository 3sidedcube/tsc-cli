use clap::{Parser, Subcommand};

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Platform {
    #[command(name = "github")]
    GitHub {
        #[arg(short, long)]
        create_repo: bool,

        #[arg(short, long, default_value = "3sidedcube")]
        org: String,

        #[arg(help = "Name of the repository")]
        repo_name: String,
    },
}

#[derive(Subcommand)]
enum Commands {
    Migrate {
        #[command(subcommand)]
        platform: Option<Platform>,
    },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Migrate { platform }) => match platform {
            Some(Platform::GitHub {
                create_repo,
                org,
                repo_name,
            }) => {
                println!("GitHub");
                if *create_repo {
                    println!("Create a repo");
                }
                println!("Pointing to {}/{}", org, repo_name);
                println!("Reconfigure current repository to new repo")
            }
            None => {}
        },
        None => {}
    }
}
