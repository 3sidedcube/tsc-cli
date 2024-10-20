use clap::{Parser, Subcommand};
use git2::Repository;

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
                let url = format!("https://github.com/{}/{}", org, repo_name);

                if *create_repo {
                    println!("Create a repo");
                }

                let repo = Repository::open(".").unwrap();
                match repo.remote_set_url("origin", &url) {
                    Ok(_) => {
                        println!("New remote URL set.")
                    }
                    Err(_) => {
                        println!("Failed to set new remote URL.")
                    }
                }

                println!("Reconfigure current repository to new repo")
            }
            None => {}
        },
        None => {}
    }
}
